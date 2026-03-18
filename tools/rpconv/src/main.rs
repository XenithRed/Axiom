use clap::Parser;
use std::path::PathBuf;

mod cmd;
mod report;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]      Io(#[from] std::io::Error),
    #[error("json: {0}")]    Json(#[from] serde_json::Error),
    #[error("zip: {0}")]     Zip(#[from] zip::result::ZipError),
    #[error("image: {0}")]   Image(#[from] image::ImageError),
    #[error("{0}")]          Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(
    name    = "rpconv",
    about   = "Convert Java Edition resource packs to Bedrock format",
    long_about = "
axiom-rpconv converts Java Edition resource packs to Bedrock Edition format.

It handles:
  - Texture path remapping (block/ → blocks/, item/ → items/)
  - Block model geometry conversion (Java elements → Bedrock bones)
  - Sound definition conversion (sounds.json → sound_definitions.json)
  - manifest.json generation

Usage:
  rpconv -i ./my-java-pack -o ./output-bedrock-pack
  rpconv -i ./pack.zip -o ./out --json report.json
"
)]
pub struct Args {
    #[arg(short, long, help = "Input: Java resource pack directory or .zip")]
    pub input: PathBuf,

    #[arg(short, long, help = "Output directory for Bedrock pack")]
    pub output: PathBuf,

    #[arg(short, long, help = "Write JSON conversion report to file")]
    pub json: Option<PathBuf>,

    #[arg(short, long, help = "Suppress output except errors")]
    pub quiet: bool,

    #[arg(long, help = "Zip the output pack after conversion")]
    pub zip: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("rpconv=info".parse().unwrap()),
        )
        .init();

    let args = Args::parse();

    if !args.quiet {
        println!();
        println!("  axiom-rpconv");
        println!("  input  : {}", args.input.display());
        println!("  output : {}", args.output.display());
        println!();
    }

    let src = if args.input.is_file()
        && args.input.extension().map(|e| e == "zip").unwrap_or(false)
    {
        match extract_zip(&args.input) {
            Ok(p) => p,
            Err(e) => { eprintln!("error extracting zip: {e}"); std::process::exit(1); }
        }
    } else {
        args.input.clone()
    };

    match cmd::convert_pack(&src, &args.output) {
        Ok(report) => {
            if !args.quiet { report.print(); }

            if let Some(ref json_path) = args.json {
                if let Ok(json) = serde_json::to_string_pretty(&report) {
                    let _ = std::fs::write(json_path, json);
                    if !args.quiet {
                        println!("  report written to {}", json_path.display());
                    }
                }
            }

            if args.zip {
                let zip_path = args.output.with_extension("mcpack");
                match zip_dir(&args.output, &zip_path) {
                    Ok(_) => {
                        if !args.quiet {
                            println!("  pack zipped to {}", zip_path.display());
                        }
                    }
                    Err(e) => eprintln!("zip error: {e}"),
                }
            }

            if report.errors() > 0 { std::process::exit(1); }
        }
        Err(e) => {
            eprintln!("conversion failed: {e}");
            std::process::exit(1);
        }
    }
}

fn extract_zip(path: &PathBuf) -> Result<PathBuf> {
    let tmp = std::env::temp_dir().join(format!("rpconv_{}", std::process::id()));
    std::fs::create_dir_all(&tmp)?;
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(&tmp)?;
    Ok(tmp)
}

fn zip_dir(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    let file = std::fs::File::create(dst)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts: zip::write::FileOptions<()> = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for entry in walkdir::WalkDir::new(src) {
        let entry = entry.map_err(|e| Error::Other(e.to_string()))?;
        let path  = entry.path();
        let rel   = path.strip_prefix(src).map_err(|e| Error::Other(e.to_string()))?;
        let name  = rel.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            let _ = zip.add_directory(&name, opts.clone());
        } else {
            zip.start_file(&name, opts.clone())?;
            let data = std::fs::read(path)?;
            use std::io::Write;
            zip.write_all(&data)?;
        }
    }
    zip.finish()?;
    Ok(())
}

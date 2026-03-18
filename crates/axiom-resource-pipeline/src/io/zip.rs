use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;
use crate::error::R;

pub struct ZipPack {
    a: HashMap<String, Vec<u8>>,
}

impl ZipPack {
    pub fn open(path: &Path) -> R<Self> {
        let file = std::fs::File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut a = HashMap::new();
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            if entry.is_file() {
                let name = entry.name().replace('\\', "/").to_string();
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                a.insert(name, data);
            }
        }
        Ok(Self { a })
    }

    pub fn read(&self, rel: &str) -> Option<&[u8]> {
        self.a.get(rel).map(|v| v.as_slice())
    }

    pub fn exists(&self, rel: &str) -> bool {
        self.a.contains_key(rel)
    }

    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.a.keys().map(|s| s.as_str())
    }

    pub fn into_map(self) -> HashMap<String, Vec<u8>> { self.a }
}

pub fn write_zip(dst: &Path, files: &HashMap<String, Vec<u8>>) -> R<()> {
    use std::io::Write;
    use zip::write::FileOptions;

    let file = std::fs::File::create(dst)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut sorted: Vec<(&str, &Vec<u8>)> = files.iter()
        .map(|(k, v)| (k.as_str(), v))
        .collect();
    sorted.sort_by_key(|(k, _)| *k);

    for (name, data) in sorted {
        zip.start_file(name, opts.clone())?;
        zip.write_all(data)?;
    }
    zip.finish()?;
    Ok(())
}

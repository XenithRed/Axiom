use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::R;

pub struct FsPack {
    a: PathBuf,
}

impl FsPack {
    pub fn open(root: &Path) -> R<Self> {
        if !root.exists() {
            return Err(crate::error::Err::NotFound(root.display().to_string()));
        }
        Ok(Self { a: root.to_path_buf() })
    }

    pub fn read(&self, rel: &str) -> R<Vec<u8>> {
        let p = self.a.join(rel);
        Ok(std::fs::read(&p)?)
    }

    pub fn read_str(&self, rel: &str) -> R<String> {
        let p = self.a.join(rel);
        Ok(std::fs::read_to_string(&p)?)
    }

    pub fn exists(&self, rel: &str) -> bool {
        self.a.join(rel).exists()
    }

    pub fn list_dir(&self, rel: &str) -> Vec<String> {
        let p = self.a.join(rel);
        if !p.is_dir() { return vec![]; }
        walkdir::WalkDir::new(&p)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter_map(|e| {
                e.path().strip_prefix(&self.a).ok()
                    .map(|r| r.to_string_lossy().replace('\\', "/"))
            })
            .collect()
    }

    pub fn all_files(&self) -> Vec<String> {
        self.list_dir("")
    }

    pub fn root(&self) -> &Path { &self.a }
}

pub fn write_all(dst: &Path, files: &HashMap<String, Vec<u8>>) -> R<()> {
    for (rel, data) in files {
        let p = dst.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&p, data)?;
    }
    Ok(())
}

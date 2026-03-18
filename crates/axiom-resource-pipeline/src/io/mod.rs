pub mod fs;
pub mod zip;

pub use fs::{write_all, FsPack};
pub use zip::{write_zip, ZipPack};

use std::collections::HashMap;
use std::path::Path;
use crate::error::R;

pub enum Pack {
    Fs(FsPack),
    Zip(ZipPack),
}

impl Pack {
    pub fn open(path: &Path) -> R<Self> {
        if path.is_file()
            && path.extension().map(|e| e == "zip" || e == "mcpack").unwrap_or(false)
        {
            Ok(Self::Zip(ZipPack::open(path)?))
        } else {
            Ok(Self::Fs(FsPack::open(path)?))
        }
    }

    pub fn read(&self, rel: &str) -> Option<Vec<u8>> {
        match self {
            Self::Fs(p)  => p.read(rel).ok(),
            Self::Zip(p) => p.read(rel).map(|b| b.to_vec()),
        }
    }

    pub fn exists(&self, rel: &str) -> bool {
        match self {
            Self::Fs(p)  => p.exists(rel),
            Self::Zip(p) => p.exists(rel),
        }
    }

    pub fn all_files(&self) -> Vec<String> {
        match self {
            Self::Fs(p)  => p.all_files(),
            Self::Zip(p) => p.files().map(|s| s.to_string()).collect(),
        }
    }
}

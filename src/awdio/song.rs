use crate::awdio::metadata::{self, Metadata};

#[derive(Debug, Default, Clone)]
pub struct Song {
    pub metadata: metadata::Metadata,
    pub path: String,
}

impl Song {
    pub fn new(path: String) -> Self {
        Song {
            metadata: metadata::Metadata::from_path(&path).unwrap(),
            path,
        }
    }

    pub fn new_temp(path: String, metadata: Metadata) -> Self {
        Song { metadata, path }
    }

    pub fn ref_array(&self) -> [&String; 3] {
        [
            &self.metadata.title,
            &self.metadata.artist,
            &self.metadata.album,
        ]
    }
}

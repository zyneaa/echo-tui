use audiotags::{MimeType, Tag};

#[derive(Debug, Default, Clone)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: u32,
    pub genre: String,
    pub track_number: u32,
    pub total_tracks: u32,
    pub disc_number: u32,
    pub total_discs: u32,
    pub album_artist: String,
    pub cover: Option<AlbumArt>,
}

#[derive(Debug, Clone)]
pub struct AlbumArt {
    pub data: Vec<u8>,
    pub mime: MimeType,
}

impl Metadata {
    pub fn from_path(path: &str) -> Result<Metadata, audiotags::Error> {
        let tag = Tag::new().read_from_path(path)?;

        Ok(Metadata {
            title: tag.title().unwrap_or("Unknown").to_string(),
            artist: tag.artist().unwrap_or("Unknown").to_string(),
            album: tag.album_title().unwrap_or("Unknown").to_string(),
            year: tag.year().unwrap_or(0) as u32,
            genre: tag.genre().unwrap_or("Unknown").to_string(),
            track_number: tag.track_number().unwrap_or(0) as u32,
            total_tracks: tag.total_tracks().unwrap_or(0) as u32,
            disc_number: tag.disc_number().unwrap_or(0) as u32,
            total_discs: tag.total_discs().unwrap_or(0) as u32,
            album_artist: tag.album_artist().unwrap_or("Unknown").to_string(),
            cover: tag.album_cover().map(|pic| AlbumArt {
                data: pic.data.into(),
                mime: pic.mime_type,
            }),
        })
    }

    pub fn update_file(&self, path: &str) -> Result<(), audiotags::Error> {
        let mut tag = Tag::default().read_from_path(path)?;

        tag.set_title(&self.title);
        tag.set_artist(&self.artist);
        tag.set_album_title(&self.album);

        tag.set_year(self.year as i32);
        tag.set_genre(&self.genre);

        tag.set_track_number(self.track_number as u16);
        if self.total_tracks > 0 {
            tag.set_total_tracks(self.total_tracks as u16);
        }

        tag.set_disc_number(self.disc_number as u16);
        if self.total_discs > 0 {
            tag.set_total_discs(self.total_discs as u16);
        }

        tag.set_album_artist(&self.album_artist);

        if let Some(art) = &self.cover {
            tag.set_album_cover(audiotags::Picture {
                mime_type: art.mime,
                data: &art.data,
            });
        } else {
            tag.remove_album_cover();
        }

        tag.write_to_path(path)?;

        Ok(())
    }
}

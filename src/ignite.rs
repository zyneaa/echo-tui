use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use directories::ProjectDirs;
use sqlx::SqlitePool;

use crate::{config::UiConfig, db, result::EchoResult};

use super::config;

pub struct Paths {
    pub config: PathBuf,
    pub data: PathBuf,
    pub songs: PathBuf,
}

impl Paths {
    fn init() -> io::Result<Self> {
        let proj = ProjectDirs::from("", "", "echo")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "couldn't find dirs"))?;

        let config = proj.config_dir();
        let data = proj.data_dir();

        for dir in [&config, &data] {
            fs::create_dir_all(dir)?;
        }

        // configs
        let config_dir = config.join("config");
        fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("echo.toml");
        if !config_file.exists() {
            fs::write(&config_file, "")?;
        }

        // data
        fs::create_dir_all(data.join("songs"))?;
        fs::create_dir_all(data.join("playlists"))?;
        let songs = data.join("songs");

        Ok(Self {
            config: config.to_path_buf(),
            data: data.to_path_buf(),
            songs,
        })
    }
}

pub async fn engine() -> EchoResult<(UiConfig, SqlitePool, Paths)> {
    let paths = Paths::init()?;

    let config_file = paths.config.join("config/echo.toml");

    let mut config: String = String::new();
    File::open(config_file)?.read_to_string(&mut config)?;

    let config_vals: config::UiConfig = toml::from_str(&config)?;
    let db_connection = db::init_db(paths.data.join("data/music.db").to_str().unwrap()).await?;

    println!("{:?}", config_vals.colors);

    let ok = (config_vals, db_connection, paths);
    Ok(ok)
}

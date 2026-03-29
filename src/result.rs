use thiserror::Error;

#[derive(Error, Debug)]
pub enum EchoReport {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Audio: {0}")]
    Audio(String),

    #[error("Audio: {0}")]
    AudioTagError(#[from] audiotags::Error),

    #[error("Metadata: {0}")]
    InvalidMetadata(String),

    #[error("Config: {0}")]
    ConfigError(String),

    #[error("Lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("Thread took too long to respond (Timeout)")]
    ThreadTimeout,

    #[error("Resource is busy, try again later")]
    ResourceBusy,

    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Config file error: {0}")]
    TOML(#[from] toml::de::Error)
}

pub type EchoResult<T> = Result<T, EchoReport>;

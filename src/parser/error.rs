use thiserror::Error;


#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ParserError {
    #[error("Unsupported file extension")]
    FileExtensionError,

    #[error("Scene file corrupted")]
    SceneCorruptedError,

    #[error("Parser error")]
    FormatCorrupted{ source: serde_json::error::Error },

    /// Represents a file read error
    #[error("Read error")]
    ReadError{ source: std::io::Error },

    // Represents any other io error
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
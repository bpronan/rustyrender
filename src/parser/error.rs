use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ParserError {
    /// A file extension that we have yet to add support for.
    #[error("Unsupported file extension")]
    FileExtension,

    /// The scene file doesn't match the schema for scene specification.
    #[error("Scene file corrupted")]
    SceneCorrupted,

    /// The file cannot be parsed by the specific file specification. For example, fbx, json, xml, etc.
    #[error("Parser error. Check that the file meets the format requirements.")]
    FormatCorrupted { source: serde_json::error::Error },

    /// Represents a file read error
    #[error(
        "Read error. Check that the file exists and that there are permissions to open the file."
    )]
    FileNotFound,

    /// Represents a file read error
    #[error(
        "Read error. Check that the file exists and that there are permissions to open the file."
    )]
    Read { source: std::io::Error },

    // Represents any other io error
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

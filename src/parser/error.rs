use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ParserError {
    /// A file extension that we have yet to add support for.
    #[error("Unsupported file extension")]
    FileExtensionError,

    /// The scene file doesn't match the schema for scene specification.
    #[error("Scene file corrupted")]
    SceneCorruptedError,

    /// The file cannot be parsed by the specific file specification. For example, fbx, json, xml, etc.
    #[error("Parser error. Check that the file meets the format requirements.")]
    FormatCorruptedError { source: serde_json::error::Error },

    /// Represents a file read error
    #[error(
        "Read error. Check that the file exists and that there are permissions to open the file."
    )]
    FileNotFoundError,

    /// Represents a file read error
    #[error(
        "Read error. Check that the file exists and that there are permissions to open the file."
    )]
    ReadError { source: std::io::Error },

    // Represents any other io error
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

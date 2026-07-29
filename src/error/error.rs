use std::{io, path::PathBuf};
use thiserror::Error;

pub type CatwalkResult<T> = std::result::Result<T, CatwalkError>;

#[derive(Debug, Error)]
pub enum CatwalkError {
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(PathBuf),

    #[error("Not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("Failed to read directory: {path}")]
    ReadDirectory {
        path: PathBuf,

        #[source]
        source: io::Error,
    },

    #[error("Failed to read file: {path}")]
    ReadFile {
        path: PathBuf,

        #[source]
        source: io::Error,
    },

    #[error("Failed to determine relative path for: {0}")]
    RelativePath(PathBuf),

    #[error("Output write failed")]
    WriteOutput(#[from] io::Error),
}

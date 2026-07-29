use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ExportedFile {
    // Path relative to the project root
    pub relative_path: PathBuf,

    // File type identifier
    pub file_type: &'static str,

    // UTF-8 contents
    pub content: String,
}

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::{
    error::{CatwalkError, CatwalkResult},
    fs::IgnoreRules,
};

pub fn collect_files(root: &Path, rules: &IgnoreRules) -> CatwalkResult<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !rules.should_ignore(entry.path()))
    {
        let entry = entry.map_err(|err| {
            let path = err.path().unwrap_or(root).to_path_buf();

            CatwalkError::ReadDirectory {
                path,
                source: std::io::Error::new(std::io::ErrorKind::Other, err),
            }
        })?;

        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    files.sort();
    Ok(files)
}

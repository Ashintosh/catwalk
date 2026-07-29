use std::fs;
use std::path::Path;

use crate::error::{CatwalkError, CatwalkResult};

pub fn read_text_file(path: &Path) -> CatwalkResult<Option<String>> {
    let bytes = fs::read(path).map_err(|source| CatwalkError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    match String::from_utf8(bytes) {
        Ok(content) => Ok(Some(content)),
        Err(_) => Ok(None),
    }
}

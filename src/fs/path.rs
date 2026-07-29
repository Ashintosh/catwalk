use std::path::{Path, PathBuf};

use crate::error::{CatwalkError, CatwalkResult};

pub fn relative_path(path: &Path, root: &Path) -> CatwalkResult<PathBuf> {
    path.strip_prefix(root)
        .map(|p| p.to_path_buf())
        .map_err(|_| CatwalkError::RelativePath(path.to_path_buf()))
}

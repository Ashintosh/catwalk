use std::path::Path;

use super::{extensions::EXTENSION_TYPES, filenames::SPECIAL_FILES};

pub fn detect(path: &Path) -> &'static str {
    let filename = path.file_name().and_then(|v| v.to_str()).unwrap_or("");

    if let Some((_, kind, _)) = SPECIAL_FILES.iter().find(|(name, _, _)| *name == filename) {
        return kind;
    }

    let extension = match path.extension().and_then(|v| v.to_str()) {
        Some(ext) => ext.to_lowercase(),
        None => return "unknown",
    };

    EXTENSION_TYPES
        .iter()
        .find(|(ext, _, _)| *ext == extension)
        .map(|(_, kind, _)| *kind)
        .unwrap_or("unknown")
}

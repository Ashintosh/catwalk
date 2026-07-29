use std::path::Path;

use crate::{
    error::CatwalkResult,
    filetype,
    fs::{self, IgnoreRules},
    model::ExportedFile,
};

use super::{OutputWriter, formatter};

pub fn export_directory<W: OutputWriter + ?Sized>(
    root: &Path,
    rules: &IgnoreRules,
    writer: &mut W,
) -> CatwalkResult<()> {
    let files = fs::collect_files(root, rules)?;

    for path in files {
        let content = match fs::read_text_file(&path)? {
            Some(value) => value,
            None => continue,
        };

        let relative = fs::relative_path(&path, root)?;

        let file = ExportedFile {
            relative_path: relative,
            file_type: filetype::detect(&path),
            content,
        };

        let output = formatter::format_file(&file);
        writer.write(&output)?;
    }

    Ok(())
}

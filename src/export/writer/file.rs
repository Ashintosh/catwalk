use std::{fs::File, io::Write, path::PathBuf};

use crate::error::CatwalkResult;

use super::OutputWriter;

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(path: PathBuf) -> CatwalkResult<Self> {
        let file = File::create(path)?;
        Ok(Self { file })
    }
}

impl OutputWriter for FileWriter {
    fn write(&mut self, content: &str) -> CatwalkResult<()> {
        self.file.write_all(content.as_bytes())?;
        Ok(())
    }
}

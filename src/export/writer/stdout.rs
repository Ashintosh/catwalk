use crate::error::CatwalkResult;

use super::OutputWriter;

pub struct StdoutWriter;
impl StdoutWriter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputWriter for StdoutWriter {
    fn write(&mut self, content: &str) -> CatwalkResult<()> {
        print!("{content}");
        Ok(())
    }
}

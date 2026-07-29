mod file;
mod stdout;

pub use file::FileWriter;
pub use stdout::StdoutWriter;

use crate::error::CatwalkResult;

pub trait OutputWriter {
    fn write(&mut self, content: &str) -> CatwalkResult<()>;
}

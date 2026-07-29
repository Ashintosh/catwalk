use std::fs;

use crate::error::{CatwalkError, CatwalkResult};

use super::Args;

pub fn validate(args: &Args) -> CatwalkResult<()> {
    if !args.path.exists() {
        return Err(CatwalkError::PathDoesNotExist(args.path.clone()));
    }

    if !fs::metadata(&args.path)?.is_dir() {
        return Err(CatwalkError::NotADirectory(args.path.clone()));
    }

    Ok(())
}

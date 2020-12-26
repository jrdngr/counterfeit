use std::path::PathBuf;

use crate::Error;

pub trait DirPicker<R> {
    fn pick_directory(&self, request: &R) -> Result<PathBuf, Error>;
}

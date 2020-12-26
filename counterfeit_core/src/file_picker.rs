use std::path::{Path, PathBuf};

use crate::Error;

pub trait FilePicker<R> {
    fn pick_file(&self, directory: &Path, request: &R) -> Result<PathBuf, Error>;
}

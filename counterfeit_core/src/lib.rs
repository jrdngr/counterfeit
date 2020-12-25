pub mod config;
pub mod dir_picker;
pub mod error;
pub mod file_picker;

pub use config::CounterfeitRunConfig;
pub use dir_picker::{DirPicker, StandardDirPicker};
pub use error::Error;
pub use file_picker::{FilePicker, StandardFilePicker};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

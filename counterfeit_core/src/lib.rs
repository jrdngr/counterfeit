pub mod config;
pub mod default;
pub mod dir_picker;
pub mod error;
pub mod file_picker;

pub use config::CounterfeitRunConfig;
pub use dir_picker::DirPicker;
pub use default::{DefaultDirPicker, DefaultFilePicker, DefaultRequest};
pub use error::Error;
pub use file_picker::FilePicker;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

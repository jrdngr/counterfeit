pub mod config;
pub mod default;
pub mod dispatcher;
pub mod error;

pub use config::CounterfeitRunConfig;
pub use default::{DefaultDirDispatcher, DefaultFileDispatcher};
pub use dispatcher::Dispatcher;
pub use error::Error;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type MultiFileIndexMap = Arc<Mutex<HashMap<PathBuf, usize>>>;

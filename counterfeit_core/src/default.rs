pub mod default_file_picker;
pub mod default_dir_picker;

pub use default_file_picker::DefaultFilePicker;
pub use default_dir_picker::DefaultDirPicker;

#[derive(Debug, Clone)]
pub struct DefaultRequest {
    pub method: String,
    pub uri_path: String,
}

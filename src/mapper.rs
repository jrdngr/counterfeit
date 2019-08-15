use std::io;

use hyper::{Body, Request, Response, StatusCode};

pub mod dir_picker;
pub mod file_picker;
pub mod mutation_handler;
pub mod response_hook;

pub use crate::mapper::dir_picker::{DirPicker, StandardDirPicker};
pub use crate::mapper::file_picker::{FilePicker, StandardFilePicker};
pub use crate::mapper::mutation_handler::{MutationHandler, ImmutableHandler};
pub use crate::mapper::response_hook::{ResponseHook, IdentityResponseHook};

use crate::{MultiFileIndexMap, CounterfeitRunConfig};

pub trait RequestMapper {
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>>;
}

pub struct FileMapper<D, F, M, R>
where
    D: DirPicker,
    F: FilePicker,
    M: MutationHandler,
    R: ResponseHook,
{
    dir_picker: D,
    file_picker: F,
    mutation_handler: M,
    response_hook: R,
}

impl<D, F, M, R> FileMapper<D, F, M, R> 
where
    D: DirPicker,
    F: FilePicker,
    M: MutationHandler,
    R: ResponseHook,
{
    pub fn new(dir_picker: D, file_picker: F, mutation_handler: M, response_hook: R) -> Self {
        Self {
            dir_picker,
            file_picker,
            mutation_handler,
            response_hook,
        }
    }
}

impl FileMapper<StandardDirPicker, StandardFilePicker, ImmutableHandler, IdentityResponseHook> {
    pub fn standard(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self {
            dir_picker: StandardDirPicker::new(config),
            file_picker: StandardFilePicker::new(index_map),
            mutation_handler: ImmutableHandler,
            response_hook: IdentityResponseHook,
        }
    }
}

impl<D, F, M, R> RequestMapper for FileMapper<D, F, M, R>
where
    D: DirPicker,
    F: FilePicker,
    M: MutationHandler,
    R: ResponseHook,
{
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>> {
        let directory = self.dir_picker.pick_directory(&request)?;
        let _file = self.file_picker.pick_file(&directory, &request)?;
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NOT_FOUND;

        self.mutation_handler.apply_mutation(&request)?;
        self.response_hook.process_response(response)
    }
}

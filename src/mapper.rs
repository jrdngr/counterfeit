use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};

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
    path_picker: D,
    file_picker: F,
    mutation_handler: M,
    response_hook: R,
}

impl<D, F, M, R> RequestMapper for FileMapper<D, F, M, R>
where
    D: DirPicker,
    F: FilePicker,
    M: MutationHandler,
    R: ResponseHook,
{
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>> {
        let directory = self.path_picker.pick_directory(&request)?;
        let _file = self.file_picker.pick_file(&directory, &request)?;
        let mut response = Response::new(Body::empty());
        *response.status_mut() = StatusCode::NOT_FOUND;

        self.mutation_handler.apply_mutation(&request)?;
        self.response_hook.process_response(response)
    }
}

pub trait DirPicker {
    fn pick_directory(&mut self, request: &Request<Body>) -> io::Result<PathBuf>;
}

pub trait FilePicker {
    fn pick_file(&mut self, directory: &Path, request: &Request<Body>) -> io::Result<PathBuf>;
}

pub trait MutationHandler {
    fn apply_mutation(&mut self, request: &Request<Body>) -> io::Result<()>;
}

pub trait ResponseHook {
    fn process_response(&mut self, response: Response<Body>) -> io::Result<Response<Body>>;
}

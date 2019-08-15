use std::fs;
use std::io;

use hyper::{Body, Request, Response, StatusCode};

pub mod dir_picker;
pub mod file_picker;
pub mod mutation_handler;
pub mod response_hook;

pub use crate::mapper::dir_picker::{DirPicker, StandardDirPicker};
pub use crate::mapper::file_picker::{FilePicker, StandardFilePicker};
pub use crate::mapper::mutation_handler::{ImmutableHandler, MutationHandler};
pub use crate::mapper::response_hook::{IdentityResponseHook, ResponseHook};

use crate::{CounterfeitRunConfig, MultiFileIndexMap};

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
    config: CounterfeitRunConfig,
}

impl<D, F, M, R> FileMapper<D, F, M, R>
where
    D: DirPicker,
    F: FilePicker,
    M: MutationHandler,
    R: ResponseHook,
{
    pub fn new(
        dir_picker: D,
        file_picker: F,
        mutation_handler: M,
        response_hook: R,
        config: CounterfeitRunConfig,
    ) -> Self {
        Self {
            dir_picker,
            file_picker,
            mutation_handler,
            response_hook,
            config,
        }
    }
}

impl FileMapper<StandardDirPicker, StandardFilePicker, ImmutableHandler, IdentityResponseHook> {
    pub fn standard(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self {
            dir_picker: StandardDirPicker::new(config.clone()),
            file_picker: StandardFilePicker::new(index_map),
            mutation_handler: ImmutableHandler,
            response_hook: IdentityResponseHook,
            config,
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
        if !self.config.silent {
            println!("Request: {} -> {}", request.method(), request.uri().path());
        }

        let directory = self.dir_picker.pick_directory(&request)?;
        let file = self.file_picker.pick_file(&directory, &request);
        self.mutation_handler.apply_mutation(&request)?;

        let mut response = Response::new(Body::empty());

        match file {
            Ok(path) => {
                *response.body_mut() = Body::from(fs::read_to_string(&path)?);
                if !self.config.silent {
                    println!("Response: 200 -> {}", path.as_path().display());
                }
            }
            Err(e) => {
                *response.body_mut() = Body::from(format!("{}", e));
                *response.status_mut() = match e.kind() {
                    io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                if !self.config.silent {
                    println!("Response: {} -> {}", response.status().as_u16(), e);
                }
            }
        }

        self.response_hook.process_response(response)
    }
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};
use hyper::header::{self, HeaderValue};

pub mod dir_picker;
pub mod file_picker;
pub mod mutation;
pub mod response_hook;

pub use crate::mapper::dir_picker::{DirPicker, StandardDirPicker};
pub use crate::mapper::file_picker::{FilePicker, StandardFilePicker};
pub use crate::mapper::mutation::ResponseMutation;
pub use crate::mapper::response_hook::{IdentityResponseHook, ResponseHook};

use crate::{CounterfeitRunConfig, MultiFileIndexMap};

pub trait RequestMapper {
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>>;
}

pub struct FileMapper<D, F, R>
where
    D: DirPicker,
    F: FilePicker,
    R: ResponseHook,
{
    dir_picker: D,
    file_picker: F,
    mutations: Vec<Box<dyn ResponseMutation>>,
    response_hook: R,
    config: CounterfeitRunConfig,
}

impl<D, F, R> FileMapper<D, F, R>
where
    D: DirPicker,
    F: FilePicker,
    R: ResponseHook,
{
    pub fn new(
        dir_picker: D,
        file_picker: F,
        mutations: Vec<Box<dyn ResponseMutation>>,
        response_hook: R,
        config: CounterfeitRunConfig,
    ) -> Self {
        Self {
            dir_picker,
            file_picker,
            mutations,
            response_hook,
            config,
        }
    }

    pub fn add_mutation(&mut self, mutation: impl ResponseMutation + 'static) {
        self.mutations.push(Box::new(mutation));
    }
}

impl FileMapper<StandardDirPicker, StandardFilePicker, IdentityResponseHook> {
    pub fn standard(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self {
            dir_picker: StandardDirPicker::new(config.clone()),
            file_picker: StandardFilePicker::new(index_map),
            mutations: Vec::new(),
            response_hook: IdentityResponseHook,
            config,
        }
    }
}

impl<D, F, R> RequestMapper for FileMapper<D, F, R>
where
    D: DirPicker,
    F: FilePicker,
    R: ResponseHook,
{
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>> {
        if !self.config.silent {
            println!("Request: {} -> {}", request.method(), request.uri().path());
        }

        let directory = self.dir_picker.pick_directory(&request)?;
        let file = self.file_picker.pick_file(&directory, &request);

        for mutation in self.mutations.iter_mut() {
            mutation.apply_mutation(&request)?;
        }

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

#[derive(Debug)]
pub struct MapperOutput<T> {
    request: Request<Body>,
    response: Response<Body>,
    metadata: T,
}

impl <T>  MapperOutput<T> {
    pub fn new(request: Request<Body>, response: Response<Body>, metadata: T) -> Self {
        Self {
            request,
            response,
            metadata,
        }
    }

    pub fn request(&self) -> &Request<Body> {
        &self.request
    }

    pub fn response(&self) -> &Response<Body> {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut Response<Body> {
        &mut self.response
    }

    pub fn metadata(&self) -> &T {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut T {
        &mut self.metadata
    }
}

impl MapperOutput<PathBuf> {
    pub fn from_file<P: AsRef<Path>>(request: Request<Body>, file_path: P) -> io::Result<Self> {
        let mut response = Response::new(Body::from(fs::read_to_string(&file_path)?));
        *response.status_mut() = StatusCode::OK;
        set_default_headers(&mut response);

        Ok(Self {
            request,
            response,
            metadata: PathBuf::from(file_path.as_ref()),
        })
    }
}

impl MapperOutput<io::Error> {
    pub fn from_error(request: Request<Body>, error: io::Error) -> Self {
        let mut response = Response::new(Body::from(format!("{}", &error)));
        *response.status_mut() = match error.kind() {
            io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        set_default_headers(&mut response);

        Self {
            request, 
            response,
            metadata: error,
        }
    }
}

fn set_default_headers(response: &mut Response<Body>) {
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("*"),
    );

    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
}

impl <T> From<MapperOutput<T>> for Response<Body> {
    fn from(mapper_output: MapperOutput<T>) -> Response<Body> {
        mapper_output.response
    }
}

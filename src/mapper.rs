use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response, StatusCode};
use hyper::header::{self, HeaderValue};

pub mod dir_picker;
pub mod file_picker;
pub mod mutation;

pub use crate::mapper::dir_picker::{DirPicker, StandardDirPicker};
pub use crate::mapper::file_picker::{FilePicker, StandardFilePicker};
pub use crate::mapper::mutation::ResponseMutation;

use crate::{CounterfeitRunConfig, MultiFileIndexMap};

pub trait RequestMapper {
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>>;
}

pub struct FileMapper<D, F>
where
    D: DirPicker,
    F: FilePicker,
{
    dir_picker: D,
    file_picker: F,
    mutations: Vec<Box<dyn ResponseMutation>>,
    config: CounterfeitRunConfig,
}

impl<D, F> FileMapper<D, F>
where
    D: DirPicker,
    F: FilePicker,
{
    pub fn new(
        dir_picker: D,
        file_picker: F,
        mutations: Vec<Box<dyn ResponseMutation>>,
        config: CounterfeitRunConfig,
    ) -> Self {
        Self {
            dir_picker,
            file_picker,
            mutations,
            config,
        }
    }

    pub fn add_mutation(&mut self, mutation: impl ResponseMutation + 'static) {
        self.mutations.push(Box::new(mutation));
    }
}

impl FileMapper<StandardDirPicker, StandardFilePicker> {
    pub fn standard(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self {
            dir_picker: StandardDirPicker::new(config.clone()),
            file_picker: StandardFilePicker::new(index_map),
            mutations: Vec::new(),
            config,
        }
    }
}

impl<D, F> RequestMapper for FileMapper<D, F>
where
    D: DirPicker,
    F: FilePicker,
{
    fn map_request(&mut self, request: Request<Body>) -> io::Result<Response<Body>> {
        if !self.config.silent {
            println!("Request: {} -> {}", request.method(), request.uri().path());
        }

        let directory = self.dir_picker.pick_directory(&request)?;
        let file = self.file_picker.pick_file(&directory, &request);
        let mut output = MapperOutput::new(request, file);

        for mutation in self.mutations.iter_mut() {
            mutation.apply_mutation(&mut output)?;
        }

        if !self.config.silent {
            println!("Response: {} -> {}", output.response.status(), output);
        }

        Ok(output.into())
    }
}

pub type MapperResult = Result<PathBuf, io::Error>;

#[derive(Debug)]
pub struct MapperOutput {
    request: Request<Body>,
    response: Response<Body>,
    result: MapperResult,
}

impl MapperOutput {
    pub fn new(request: Request<Body>, result: MapperResult) -> Self {
        let response = match &result {
            Ok(path) => Self::response_from_file(path),
            Err(e) => Self::response_from_error(e),
        };

        Self {
            request,
            response,
            result,
        }
    }

    fn response_from_file<P: AsRef<Path>>(file_path: P) -> Response<Body> {
        match fs::read_to_string(&file_path) {
            Ok(path) => {
                let mut response = Response::new(Body::from(path));
                *response.status_mut() = StatusCode::OK;
                set_default_headers(&mut response);
                response
            },
            Err(e) => Self::response_from_error(&e),
        }
    }

    fn response_from_error(error: &io::Error) -> Response<Body> {
        let mut response = Response::new(Body::from(format!("{}", error)));
        *response.status_mut() = match error.kind() {
            io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        set_default_headers(&mut response);
        response
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

    pub fn result(&self) -> &MapperResult {
        &self.result
    }

    pub fn result_mut(&mut self) -> &mut MapperResult {
        &mut self.result
    }
}

impl std::fmt::Display for MapperOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.result {
            Ok(path) => write!(f, "{}", path.display()),
            Err(e) => write!(f, "{}", e),
        }
    }
}

impl From<MapperOutput> for Response<Body> {
    fn from(mapper_output: MapperOutput) -> Response<Body> {
        mapper_output.response
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

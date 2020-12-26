use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{fs, marker::PhantomData};

use anyhow::Result;
use counterfeit_core::{
    DefaultDirPicker, DefaultFilePicker, DefaultRequest, DirPicker, Error, FilePicker,
};
use futures::future;
use hyper::header::{self, HeaderValue};
use hyper::service::Service;
use hyper::{Body, Request, Response, StatusCode};

use crate::{CounterfeitRunConfig, MultiFileIndexMap};

pub struct FileMapperService<D, F, R>
where
    D: DirPicker<R>,
    F: FilePicker<R>,
{
    dir_picker: D,
    file_picker: F,
    config: CounterfeitRunConfig,
    _request: PhantomData<R>,
}

impl<D, F, R> FileMapperService<D, F, R>
where
    D: DirPicker<R>,
    F: FilePicker<R>,
{
    pub fn new(dir_picker: D, file_picker: F, config: CounterfeitRunConfig) -> Self {
        Self {
            dir_picker,
            file_picker,
            config,
            _request: PhantomData,
        }
    }
}

impl FileMapperService<DefaultDirPicker, DefaultFilePicker, DefaultRequest> {
    pub fn default(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self {
            dir_picker: DefaultDirPicker::new(config.clone()),
            file_picker: DefaultFilePicker::new(config.create_missing, index_map),
            config,
            _request: PhantomData,
        }
    }
}

impl<D, F> Service<Request<Body>> for FileMapperService<D, F, DefaultRequest>
where
    D: DirPicker<DefaultRequest>,
    F: FilePicker<DefaultRequest>,
{
    type Response = Response<Body>;
    type Error = anyhow::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        if !self.config.silent {
            println!("Request: {} -> {}", request.method(), request.uri().path());
        }

        let default_request = create_default_request(&request);

        match self.dir_picker.pick_directory(&default_request) {
            Ok(directory) => {
                let file = self.file_picker.pick_file(&directory, &default_request);
                let output = MapperOutput::new(request, file);

                if !self.config.silent {
                    println!("Response: {} -> {}", output.response.status(), output);
                }

                future::ok(output.into())
            }
            Err(e) => future::err(e.into()),
        }
    }
}

pub struct MakeFileMapperService {
    config: CounterfeitRunConfig,
    index_map: MultiFileIndexMap,
}

impl MakeFileMapperService {
    pub fn new(config: CounterfeitRunConfig, index_map: MultiFileIndexMap) -> Self {
        Self { config, index_map }
    }
}

impl<T> Service<T> for MakeFileMapperService {
    type Response = FileMapperService<DefaultDirPicker, DefaultFilePicker, DefaultRequest>;
    type Error = anyhow::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(FileMapperService::default(
            self.config.clone(),
            Arc::clone(&self.index_map),
        ))
    }
}

pub type MapperResult = Result<PathBuf, Error>;

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
            }
            Err(e) => Self::response_from_error(&Error::IoError(e)),
        }
    }

    fn response_from_error(error: &Error) -> Response<Body> {
        let mut response = Response::new(Body::from(format!("{}", error)));
        *response.status_mut() = match error {
            Error::IoError(io_error) if io_error.kind() == io::ErrorKind::NotFound => {
                StatusCode::NOT_FOUND
            }
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

fn create_default_request(request: &Request<Body>) -> DefaultRequest {
    DefaultRequest {
        method: request.method().to_string(),
        uri_path: request.uri().path().to_string(),
    }
}

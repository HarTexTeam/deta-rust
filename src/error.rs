use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

use hyper::http::Error as HttpError;
use hyper::Error as HyperError;
use serde::Deserialize;
use serde_json::Error as JsonError;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub errors: Vec<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.errors)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ErrorKind::ApiError { src } => write!(f, "api error(s): {src}"),
            ErrorKind::HttpError { src } => write!(f, "http error: {src}"),
            ErrorKind::HyperError { src } => write!(f, "hyper error: {src}"),
            ErrorKind::JsonError { src } => write!(f, "json error: {src}"),
        }
    }
}

impl From<ApiError> for Error {
    fn from(src: ApiError) -> Self {
        Self {
            kind: ErrorKind::ApiError { src }
        }
    }
}

impl From<HttpError> for Error {
    fn from(src: HttpError) -> Self {
        Self {
            kind: ErrorKind::HttpError { src }
        }
    }
}

impl From<HyperError> for Error {
    fn from(src: HyperError) -> Self {
        Self {
            kind: ErrorKind::HyperError { src }
        }
    }
}

impl From<JsonError> for Error {
    fn from(src: JsonError) -> Self {
        Self {
            kind: ErrorKind::JsonError { src }
        }
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    ApiError { src: ApiError },
    HttpError { src: HttpError },
    HyperError { src: HyperError },
    JsonError { src: JsonError },
}

pub type Result<T> = StdResult<T, Error>;

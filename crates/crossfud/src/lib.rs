#![allow(non_snake_case)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

use core::fmt;
use std::{fmt::{Display, Formatter}, any::Any};

use easydev::builder::*;
use reqwest::{header::{HeaderMap, COOKIE}, Url};

pub mod drivers;
pub mod clients;
pub use drivers::*;
pub use clients::*;

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub enum Error {
	/// An Error returned by the API
	ApiError(String),
	/// An Error not related to the API
	RequestError(String),
    /// An Error  related to file downloading
    DownloadError(String),
    /// An Error  related to io operations
    IoError(String),
    /// An Error  related to file saving
    FileError(String),
    /// An Error  related to file saving
    DriverError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(msg) => write!(f, "API error: {}", msg),
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
            Error::DownloadError(msg) => write!(f, "Download error: {}", msg),
            Error::IoError(msg) => write!(f, "Io error: {}", msg),
            Error::FileError(msg) => write!(f, "File error: {}", msg),
            Error::DriverError(msg) => write!(f, "Driver error: {}", msg),
        }
    }
}

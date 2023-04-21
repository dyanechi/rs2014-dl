#![allow(non_snake_case)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

use core::fmt;
use std::fmt::{Display, Formatter};

use easydev::builder::*;
use reqwest::header::{HeaderMap, COOKIE};

pub mod fetcher;
pub use fetcher::*;

pub mod drivers;
pub use drivers::*;

pub trait LoadFromCache {
    type Output;
    fn load_from_cache() -> ApiResult<Self::Output>;
}

pub trait SaveToCache {
    fn save_to_cache() -> ApiResult<()>;
}



pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

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

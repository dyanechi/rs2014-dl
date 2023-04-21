use crate::*;
use super::*;

mod downloader;
mod uploader;
mod fetcher;
pub mod traits;

pub use downloader::*;
pub use uploader::*;
pub use fetcher::*;
// pub use traits::*;


fn driver_not_implemented<T>() -> ApiResult<T> {
    Err(Error::DriverError("Driver not implemented".to_string()))
}
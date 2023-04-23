use crate::*;
use super::*;
use serde::de::DeserializeOwned;


mod downloader;
mod uploader;
mod fetcher;
mod cacher;
mod traits;
// pub mod traits;

pub use downloader::*;
pub use uploader::*;
pub use fetcher::*;
pub use cacher::*;
pub use traits::*;

// pub use traits::*;

impl<T: Driver> Driver for Box<T> {
    const DOMAIN_URL: &'static str = T::DOMAIN_URL;
}
impl<T: DownloadDriver> DownloadDriver for Box<T> {
    const DL_URL: &'static str = T::DL_URL;
    const DOWNLOAD_DIR: &'static str = T::DOWNLOAD_DIR;
    type DlResult = T::DlResult;
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
        where 
            U: Into<String>,
            F: Into<String>, 
        { self.as_ref().download(url, file_name) }
}  


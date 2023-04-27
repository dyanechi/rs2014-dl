use crate::*;
use super::*;
use std::{any::Any, path::{Path, PathBuf}};
use directories::*;
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

#[derive(Default, WithBuilder)]
pub struct FullClient {
    uploader: Option<Uploader>,
    downloader: Option<Downloader>,
    cacher: Option<Cacher>,
    fetcher: Option<Fetcher>,
}

impl FullClient {
    pub fn uploader(&self) -> &Option<Uploader> { &self.uploader }
    pub fn downloader(&self) -> &Option<Downloader> { &self.downloader }
    pub fn cacher(&self) -> &Option<Cacher> { &self.cacher }
    pub fn fetcher(&self) -> &Option<Fetcher> { &self.fetcher }
}

// pub struct FullClient<U: UploadDriver, C: CacheDriver, F: FetchDriver> {
//     uploader: Option<Uploader<U>>,
//     downloader: Option<Downloader>,
//     cacher: Option<Cacher<C>>,
//     fetcher: Option<Fetcher<F>>,
// }
// impl<U: UploadDriver, C: CacheDriver, F: FetchDriver> FullClient<U, C, F> {
//     pub fn new() -> FullClientBuilder<U, C, F> {
//         FullClientBuilder::new()
//     }
// }

// struct FullClientBuilder<U: UploadDriver = Uploader<>, C: CacheDriver, F: FetchDriver> {
//     inner: FullClient<U, C, F>,
// }
// impl<U: UploadDriver, C: CacheDriver, F: FetchDriver> FullClientBuilder<U, C, F> {
//     fn new() -> Self {
//         Self { inner: FullClient { uploader: None, downloader: None, cacher: None, fetcher: None } }
//     }
//     pub fn with_uploader(mut self, uploader: Uploader<U>) -> Self {
//         self.inner.uploader = Some(uploader); self
//     }
//     pub fn with_downloader(mut self, downloader: Downloader) -> Self {
//         self.inner.downloader = Some(downloader); self
//     }
//     pub fn with_fetcher(mut self, fetcher: Fetcher<F>) -> Self {
//         self.inner.fetcher = Some(fetcher); self
//     }
//     pub fn with_cacher(mut self, cacher: Cacher<C>) -> Self {
//         self.inner.cacher = Some(cacher); self
//     }
//     pub fn build(self) -> FullClient<U, C, F> {
//         self.inner
//     }
// }



// impl<T: Driver> Driver for Box<T> {
//     const DRIVER_NAME: &'static str = "";
//     const DOMAIN_URL: &'static str = T::DOMAIN_URL;
// }
// impl<T: DownloadDriver> DownloadDriver for Box<T> {
//     const DL_URL: &'static str = T::DL_URL;
//     const DOWNLOAD_DIR: &'static str = T::DOWNLOAD_DIR;
//     type DlResult = T::DlResult;
//     fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
//         where 
//             U: Into<String>,
//             F: Into<String>, 
//         { self.as_ref().download(url, file_name) }
// }  


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let client = FullClient::new()
            .with_downloader(Downloader::new())
            .build();
    
        let url = "https://drive.google.com/file/d/1iGB6tvPMt3VY-a8xH5mTrnFfaN3KvNxM/view";
        let file_name = "test_download_p.psarc";
        let downloader = client.downloader().as_ref().unwrap();
        let download_dir = Downloader::download_dir(); 
        match downloader.download_from_url(url, file_name) {
            Ok(result) => println!("file downloaded to {}", download_dir.join(&result.title).display()),
            Err(error) => panic!("{}", error.to_string())
        }
    }
}


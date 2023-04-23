use serde::de::DeserializeOwned;

use super::*;

fn driver_not_implemented<T>() -> ApiResult<T> {
    Err(Error::DriverError("Driver not implemented".to_string()))
}

// Functional Traits
// ----------------------------------------------------------------

pub trait Driver {
    /// A host website domain
    const DOMAIN_URL: &'static str;
}

// Process Traits
// ----------------------------------------------------------------
pub trait ProcessDriver: Any + AsAny + Driver + DownloadDriver + UploadDriver + FetchDriver {
    type DriverType: ProcessDriver;
    fn create() -> ClientInstance<Self::DriverType>;
}

// Downloader Traits
// ----------------------------------------------------------------

pub trait DownloadDriver: Driver {
    /// URL to download files from
    const DL_URL: &'static str = Self::DOMAIN_URL;
    const DOWNLOAD_DIR: &'static str = "./downloads";
    type DlResult = ();
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
    where 
        U: Into<String>,
        F: Into<String>,
    { driver_not_implemented() }
}

pub trait GetDownloader: DownloadDriver
where Box<Self>: DownloadDriver {
    fn downloader(&self) -> Downloader<Box<Self>>;
}

// Fetch Traits
// ----------------------------------------------------------------

pub trait FetchDriver: Driver {
    /// URL to fetch files from
    const FT_URL: &'static str = Self::DOMAIN_URL;
    const FT_QUERY: &'static str = "";
    fn fetch<T: DeserializeOwned>(&self) -> ApiResult<T> { driver_not_implemented() }
}

pub trait GetFetcher: FetchDriver
where Box<Self>: FetchDriver {
    fn fetcher(&self) -> Fetcher<Box<Self>>;
}

// Upload Traits
// ----------------------------------------------------------------

pub trait UploadDriver: Driver {
    /// URL to upload files to
    const UP_URL: &'static str = Self::DOMAIN_URL;
    type UpResult = ();
    fn upload(&self) -> ApiResult<Self::UpResult> { driver_not_implemented() }
}

pub trait GetUploader: UploadDriver
where Box<Self>: UploadDriver {
    fn uploader(&self) -> Uploader<Box<Self>>;
}

// Cache Traits
// ----------------------------------------------------------------

// pub trait LoadFromCache {
//     type Output;
// }

// pub trait SaveToCache {
// }

pub trait CacheDriver: Driver {
    const CACHE_DIR: &'static str;
    fn load_from_cache<T>() -> ApiResult<T>;
    fn save_to_cache() -> ApiResult<()>;
}


pub type RequestDriver<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> = Box<T>;

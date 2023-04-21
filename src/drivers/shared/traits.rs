use serde::de::DeserializeOwned;

use super::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
pub trait ProcessDriver: Any + AsAny + DownloadDriver + UploadDriver + FetchDriver {
    type DriverType: ProcessDriver;
    fn create() -> DriverInstance<Self::DriverType>;
}
pub trait DownloadDriver {
    const DOWNLOAD_DIR: &'static str = "./downloads";
    type DlResult;
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
    where 
        U: Into<String>,
        F: Into<String>,
    { driver_not_implemented() }
}

pub trait FetchDriver {
    fn fetch<T: DeserializeOwned>(&self) -> ApiResult<T> { driver_not_implemented() }
}

pub trait UploadDriver {
    type UpResult;
    fn upload(&self) -> ApiResult<Self::UpResult> { driver_not_implemented() }
}


pub type RequestDriver<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> = Box<T>;

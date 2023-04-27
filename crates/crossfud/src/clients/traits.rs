use std::ops::Deref;

use super::*;
use crate::{drivers::traits::*, ApiResult};

#[derive(Default)]
pub struct ClientConfig {}
pub trait Client {
    const CLIENT_NAME: &'static str;

    fn config(&self) -> ClientConfig {
        ClientConfig::default()
    }
}


pub struct DownloadResult(FileMeta);
impl Deref for DownloadResult {
    type Target = FileMeta;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub trait DownloadClient: Client {
    // fn download(&self) -> ApiResult<()> {

    // }
    fn download_from_url(&self, url: &str, file_name: &str) -> ApiResult<DownloadResult> {
        println!("[INFO] Downloading file");
        let driver = DriverKind::from_url(url)?;
        let result: DownloadedFile; 
        match driver {
            DriverKind::DropBox(driver) => result = driver.download(url)?,
            DriverKind::GDrive(driver) => result = driver.download(url)?,
            DriverKind::Mega(driver) => result = driver.download(url)?,
        }

        let file_path = Self::download_dir().join(result.metadata().title);
        if file_path.exists() {
            return Err(Error::FileError(format!("file '{file_name}' already exists")));
        }
        if !file_path.parent().unwrap().exists() {
            std::fs::create_dir_all(&file_path.parent().unwrap()).expect("failed creating download directory");
        }

        if let Err(err) = std::fs::write(&file_path, result.bytes()) {
            return Err(Error::FileError(err.to_string()));
        };
        Ok(DownloadResult(result.metadata()))
    }

    /// Returns download directory path 
    fn download_dir() -> PathBuf {
        let user_dirs = UserDirs::new().expect("failed fetching user directories");
        let download_dir = user_dirs.download_dir().expect("failed getting downloads directory");
        download_dir.join(Self::CLIENT_NAME)
    }
}
pub trait UploadClient: Client {}
pub trait FetchClient: Client {
    fn fetch<T: DeserializeOwned>(&self) -> ApiResult<T>;
}
pub trait CacheClient: Client {
    fn cache_dir(&self) -> PathBuf {
        ProjectDirs::from("com", "crossfud", Self::CLIENT_NAME)
            .expect("failed fetching user directories")
            .cache_dir()
            .to_path_buf()
    }
}
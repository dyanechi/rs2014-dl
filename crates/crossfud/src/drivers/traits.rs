use std::{path::{PathBuf, Path}, str::FromStr, io::{BufReader, BufWriter}};

use serde::{de::DeserializeOwned, Serialize};

use super::*;

fn driver_not_implemented<T>() -> ApiResult<T> {
    Err(Error::DriverError("Driver not implemented".to_string()))
}

// Functional Traits
// ----------------------------------------------------------------

pub trait Driver {
    /// A domain without scheme or path
    /// ex: drive.google.com
    const DOMAIN_URL: &'static str;

    /// Driver name
    const DRIVER_NAME: &'static str;

    // fn domain(&self) -> Option<String> {
    //     match Url::from_str(Self::DOMAIN_URL) {
    //         Ok(url) => {
    //             let Some(domain) = url.domain() else {
    //                 return None;
    //             };
    //             Some(Url::from_str(&format!("https://{}", domain)).unwrap().to_string())
    //         },
    //         Err(_) => None
    //     }

    // }

    fn create() -> Self;
    fn driver_meta(&self) -> DriverMeta {
        DriverMeta {
            remote_host: RemoteHost::new(&format!("https://{}",Self::DOMAIN_URL))
        }
    }
}

// Process Traits
// ----------------------------------------------------------------
// pub trait ProcessDriver: Any + AsAny + Driver + DownloadDriver + UploadDriver + FetchDriver {
//     type DriverType: ProcessDriver;
//     fn create() -> ClientInstance<Self::DriverType>;
// }

// Downloader Traits
// ----------------------------------------------------------------

struct DownloadProps {
    url: String,
    file_name: String,
}

pub struct DownloadedFile {
    metadata: FileMeta,
    bytes: Vec<u8>,
}
impl DownloadedFile {
    pub fn new(metadata: FileMeta, bytes: Vec<u8>) -> Self {
        Self { metadata, bytes }
    }
    pub fn metadata(&self) -> FileMeta {
        self.metadata.clone()
    }
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
pub trait DownloadDriver: Driver {
    /// URL to download files from
    const DOWNLOAD_URL: &'static str = Self::DOMAIN_URL;
    const DOWNLOAD_DIR: &'static str = "./downloads";
    type DlResult = DownloadedFile;
    fn download<'a>(&self, url: &'a str) -> ApiResult<Self::DlResult>;
}

// pub trait GetDownloader: DownloadDriver
// where Box<Self>: DownloadDriver {
//     fn downloader(&self) -> Downloader<Box<Self>>;
// }

// Fetch Traits
// ----------------------------------------------------------------

pub trait FetchDriver: Driver {
    /// URL to fetch files from
    const FT_URL: &'static str = Self::DOMAIN_URL;
    const FT_QUERY: &'static str = "";
    fn fetch<T: DeserializeOwned>(&self) -> ApiResult<T> { driver_not_implemented() }
    fn fetch_file_meta(&self) -> ApiResult<FileMeta> {
        
    }
}

pub trait GetFetcher: FetchDriver
where Box<Self>: FetchDriver {
    // fn fetcher(&self) -> Fetcher<Box<Self>>;
    fn fetcher(&self) -> Fetcher;
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
    // fn uploader(&self) -> Uploader<Box<Self>>;
    fn uploader(&self) -> Uploader;
}

// Cache Traits
// ----------------------------------------------------------------

pub trait CacheDriver: Driver {
    const CACHE_APP_NAME: &'static str;
    fn cache_dir() -> ApiResult<PathBuf> {
        match directories::ProjectDirs::from("", "dyanechi", "crossfud") {
            Some(project_dirs) => Ok(project_dirs.cache_dir().join(Self::CACHE_APP_NAME).to_path_buf()),
            None => Err(Error::IoError("Couldn't find cache directory".to_string())),
        }
    }
    fn load_from_cache<T: DeserializeOwned>(file: PathBuf) -> ApiResult<Option<T>> {
        use std::fs;
        let file_name = file.file_name().unwrap();
        let path = Self::cache_dir()?.join(&file_name).join(".json");
        // if !path.exists() { return Err(Error::FileError(format!("path '{}' doesn't exist", path.display()))); }
        if !path.exists() { return Ok(None); }
        if !path.is_file() { return Err(Error::FileError(format!("path '{}' is not a valid file", path.display()))); }

        let Ok(file) = fs::File::open(&path) else {
            return Err(Error::FileError(format!("file '{}' couldn't be opened", path.display())));
        };
        let rdr = BufReader::new(file);

        match serde_json::from_reader(rdr) {
            Ok(json) => {
                println!("Loaded data from cache '{}'", path.display());
                Ok(Some(json))
            },
            Err(error) => Err(Error::FileError(error.to_string()))
        }

        // Err(Error::IoError("Couldn't open cache file".to_string()))

    }
    fn save_to_cache<T: Serialize>(file: PathBuf, json: T) -> ApiResult<()> {
        use std::fs;
        use std::io::Write;
        let Some(file_name) = file.file_name() else {
            return Err(Error::FileError(format!("file '{}' is invalid", file.display())));
        };

        let path = Self::cache_dir()?.join(file_name);
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => println!("Created directory '{}'", parent.display()),
                Err(error) => return Err(Error::FileError(error.to_string())),
            }
        }

        if !path.exists() {
            fs::File::create(&path).unwrap();
        }
        let writer = fs::File::open(&path).unwrap();
        
        match serde_json::to_writer(writer, &json) {
            Ok(_) => {
                println!("Saved data to cache '{}'", path.display());
                Ok(())
            },
            Err(error) => return Err(Error::FileError(error.to_string()))
        }
    }
}


// pub type RequestDriver<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> = Box<T>;

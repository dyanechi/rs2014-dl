use std::{path::{Path, PathBuf}, collections::HashMap};

use serde::Serialize;

use super::*;

pub struct Downloader {

    // remote: RemoteHost,
    // url: RemoteHost,
}
impl Downloader {
    const DOWNLOAD_DIR: &'static str = "./downloads";
    pub fn new() -> Downloader {
        Self {
            // remote: RemoteHost::default(),
            // url: RemoteHost::default()
        }
    }
}

impl Client for Downloader {
    const CLIENT_NAME: &'static str = "Crossfud";
}
impl DownloadClient for Downloader {}

impl Downloader {
    // pub fn download(url: &str, file_name: &str) -> ApiResult<()> {
    //     let file_path = Path::new(Self::DOWNLOAD_DIR).join(file_name);
    //     std::fs::create_dir_all(&file_path.parent().unwrap()).unwrap();

    //     match DlDriver::from_url(url)? {
    //         DlDriver::GDrive(driver) => driver.download(url, file_name),
    //         DlDriver::DropBox(driver) => driver.download(url, file_name),
    //         DlDriver::Mega(driver) => driver.download(url, file_name),
    //     }
        
    //     // if file_path.exists() { return Ok(DownloadState::Skipped ); }

    //     // let download_url = self.parse_download_url(url)?;

    //     // println!("[INFO] Downloading from '{download_url}'...");]
    //     // match reqwest::blocking::get(url) {
    //     //     Ok(resp) => {
    //     //         let body = resp.bytes().unwrap();
    //     //         if let Err(err) = std::fs::write(file_path, &body) {
    //     //             return Err(Error::FileError(err.to_string()));
    //     //         };
    //     //         Ok(())
    //     //     },
    //     //     Err(error) => Err(Error::RequestError(format!("{}", error.to_string())))
    //     // }

    //     // Ok(DownloadState::Downloaded)
    //     // Err(Error::DriverError(format!("Failed")))
    // }

    pub(crate) fn get(url: &str) -> ApiResult<Vec<u8>> {
        println!("downloading GET file from {}...", url);
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                let bytes = resp.bytes().unwrap();
                if bytes.is_empty() {
                    return Err(Error::RequestError(format!("requested file is empty")))
                }
                Ok(bytes.to_vec())
            },
            Err(error) => Err(Error::RequestError(format!("{}", error.to_string())))
        }
    }

    pub(crate) fn post<T: Serialize>(url: &str, json: &T, file_path: PathBuf) -> ApiResult<()> {
        let client = reqwest::blocking::Client::new();
        let request = client.post(url).json(json).build().expect("should build request");
        match client.execute(request) {
            Ok(resp) => {
                let body = resp.bytes().unwrap();
                if let Err(err) = std::fs::write(file_path, &body) {
                    return Err(Error::FileError(err.to_string()));
                };
                Ok(())
            },
            Err(error) => Err(Error::RequestError(format!("{}", error.to_string())))
        }
    }

    pub(crate) fn check_file_exists(file_name: &str) -> ApiResult<()> {
        if Path::new(&Self::download_dir()).join(file_name).exists() {
            return Err(Error::FileError(format!("File '{file_name}' already exists")))
        }
        Ok(())
    }
    // pub(crate) fn execute(request: reqwest::Request) -> ApiResult<()> {

    // }
}

// impl<T: DownloadDriver> Download for DlDriver<T> {
//     fn download(&self) -> ApiResult<()> {
//         self.driver.download();
//         Ok(())
//     }
// }

// impl DownloadDriver for Driver {}
// impl Download for Driver {
//     fn download(&self) -> ApiResult<()> {
        
//         match self {
//             Driver::Ignition(driver) => driver,
//         }.download()
//         // Err(Error::DriverError(format!("")))
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;


}
use std::path::Path;

use super::*;



// pub struct Custom {}
// impl DownloadDriver for Custom {
//     type DlResult = ();
//     fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
//         where 
//             U: Into<String>,
//             F: Into<String>, {
//         Err(Error::DownloadError("".to_string()))
//     }
// }
// impl Driver<DlDriver<Custom>> for Custom {
//     fn create() -> DlDriver<Custom> {
//         DlDriver {
//             driver: Custom { },
//             remote: RemoteHost::default(),
//             url: RemoteUrl::default()
//         }
//     }
// }

pub enum DlDriverKind {
    GDrive(Downloader<GDrive>),
    DropBox(Downloader<DropBox>),
    // Custom(DlDriver<Custom>),
    Unknown,
}
impl DlDriverKind {

    // pub fn self_custom(custom_driver: DlDriver<Custom>) -> Self {
    //     Self::Custom(custom_driver)
    // }

    // pub fn self_from_url(url: &str) -> ApiResult<Self> {
    //     let parsed_url = Url::parse(&url).expect("Failed to parse url");
    //     let domain = parsed_url.domain().expect("Expected domain in url");
    //     match domain {
    //         "drive.google.com" => Ok(Self::GDrive(GDrive::create())),
    //         "www.dropbox.com" => Ok(Self::DropBox(DropBox::create())),
    //         _ => Err(Error::DownloadError(format!("Unsupported domain: '{domain}'")) )
    //     }
    // }

    fn parse_url(parsed_url: &Url) -> ApiResult<RemoteHost> {
        let domain = parsed_url.domain().unwrap();
        match domain {
            "drive.google.com" => {
                for (k, v) in parsed_url.query_pairs(){
                    if k == "amp;resourcekey" {
                        return Err(Error::DownloadError(format!("Link requires Google authorization: {}", parsed_url)))
                    }
                    if k == "id" {
                        return Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", v)));
                    }
                };

                let mut uri = parsed_url.path_segments().unwrap();
                uri.next();
                if let Some(sub_location) = uri.next() {
                    if sub_location == "folders" {
                        return Err(Error::DownloadError("Downloading from folders not supported".to_string()));
                    }
                };
                
                match uri.next() {
                    Some(id) => Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", id))),
                    None => Err(Error::DownloadError("Can't find id in url".to_string()))
                }
            },
            "www.dropbox.com" => {
                let mut url = parsed_url.clone();
                url.set_query(Some("dl=1"));
                url.to_string();
                Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", url.as_str())))
            },
            // "mega.nz" => {
            //     return Err(Error::DownloadError(format!("Domain not supported: '{domain}'")) )
            // },
            _ => return Err(Error::DownloadError(format!("Unsupported domain: '{domain}'")) )
        }
    }

    // pub fn from_url(url: &str) -> ApiResult<RemoteHost> {
    //     let parsed_url = Url::parse(&url).expect("Failed to parse url");
    //     let mut remote_host = RemoteHost::default();
    //     match Self::parse_url(&parsed_url) {
    //         Ok(url) => Ok(RemoteHost::new(url)),
    //         Err(error) => Err(Error::DownloadError(error.to_string()))
    //     }
    // }
}




pub struct Downloader<T: DownloadDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteHost,
}
impl<T: DownloadDriver> Downloader<T> {
    pub fn new(driver: T) -> Downloader<T> {
        Self {
            driver,
            remote: RemoteHost::default(),
            url: RemoteHost::default()
        }
    }
}

impl<T: DownloadDriver> Downloader<T> {
    pub fn download(url: &str, file_name: &str) -> ApiResult<()> {
        let file_path = Path::new(T::DOWNLOAD_DIR).join(file_name);
            // if file_path.exists() { return Ok(DownloadState::Skipped ); }
            std::fs::create_dir_all(&file_path.parent().unwrap()).unwrap();
    
            // let download_url = self.parse_download_url(url)?;
    
            // println!("[INFO] Downloading from '{download_url}'...");]
            match reqwest::blocking::get(url) {
                Ok(resp) => {
                    let body = resp.bytes().unwrap();
                    if let Err(err) = std::fs::write(file_path, &body) {
                        return Err(Error::FileError(err.to_string()));
                    };
                    Ok(())
                },
                Err(error) => Err(Error::RequestError(format!("{}", error.to_string())))
            }
    
            // Ok(DownloadState::Downloaded)
            // Err(Error::DriverError(format!("Failed")))
    }
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

    #[test]
    fn test_download_drivers() {
        let url = "https://drive.google.com/uc?id={}";
        let driver = GDrive {};
        driver.download("", "file_name").unwrap();
        // let driver = DlDriverKind::self_from_url(url).expect("should get dl_driver");

        // if let DlDriverKind::DropBox(driver) = driver {
            
        // } 
    }
}
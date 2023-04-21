use super::*;



pub enum DlRemoteHost {
    GDrive,
    DropBox,
    Custom(RemoteHost),
    Unknown,
}
impl DlRemoteHost {
    pub fn from_url(url: &str) -> ApiResult<RemoteHost> {
        let remote_host = RemoteHost::default();
        let parsed_url = Url::parse(&url).expect("Failed to parse url");
        let domain = parsed_url.domain().unwrap();
        match domain {
            "drive.google.com" => {
                let mut id = format!("");
                for (k, v) in parsed_url.query_pairs(){
                    if k == "id" { id = v.to_string(); break }
                    if k == "amp;resourcekey" {
                        return Err(Error::DownloadError(format!("Link requires Google authorization: {}", parsed_url)))
                    }
                };
                if !id.is_empty() {
                    format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", id);
                    Ok(RemoteHost::default())
                } else {
                    let mut uri = parsed_url.path_segments().unwrap();
                    uri.next();
                    if let Some(sub_location) = uri.next() {
                        if sub_location == "folders" {
                            return Err(Error::DownloadError("Downloading from folders not supported".to_string()));
                        }
                        // return Err(Error::DownloadError("Should have sub-location".to_string()));
                    };
                    
                    let Some(id) = uri.next() else {
                        return Err(Error::DownloadError("Can't find id in url".to_string()));
                    };
                    format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", id);
                    Ok(RemoteHost::default())
                }
            },
            "mega.nz" => {
                return Err(Error::DownloadError(format!("Domain not supported: '{domain}'")) )
            },
            "www.dropbox.com" => {
                let mut url = parsed_url.clone();
                url.set_query(Some("dl=1"));
                url.to_string();
                Ok(RemoteHost::default())
            },
            _ => return Err(Error::DownloadError(format!("Unrecognized domain: '{domain}'")) )
        }



        // Err(Error::DriverError(format!("Unsupported")))
    }
}

pub struct DlDriver<T: DownloadDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteUrl,
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
use super::*;

#[derive(Clone)]
pub struct GDrive {}

impl Driver for GDrive {
    const DOMAIN_URL: &'static str = "https://drive.google.com";
}
impl DownloadDriver for GDrive {
    type DlResult = ();
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
        where 
            U: Into<String>,
            F: Into<String>, {
        
        let parsed_url = Url::parse(&url.into()).unwrap();
        let remote_host = remote_url(parsed_url)?;

        Downloader::<Self>::download(&remote_host.url, &file_name.into())
        // let downloader = self.downloader();
        // let dldriver = Downloader::new(GDrive {});
        
        // Err(Error::DownloadError("".to_string()))
    }
}
impl GetDownloader for GDrive {
    fn downloader(&self) -> Downloader<Box<Self>> {
        Downloader::new(Box::new(self.clone()))
    }
}


fn remote_url(url: Url) -> ApiResult<RemoteHost> {
    for (k, v) in url.query_pairs(){
        if k == "amp;resourcekey" {
            return Err(Error::DownloadError(format!("Link requires Google authorization: {}", url)))
        }
        if k == "id" {
            return Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", v)));
        }
    };

    let mut uri = url.path_segments().unwrap();
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
}
// impl Driver<DlDriver<GDrive>> for GDrive {
//     fn create() -> DlDriver<GDrive> {
//         DlDriver::new(GDrive {  })
//     }
// }

// impl Driver<DlDriver<GDrive>> for GDrive {}
// impl DwDriver<GDrive> for GDrive {
//     fn downloader(&self) -> DlDriver<GDrive> {
        
//     }
// }

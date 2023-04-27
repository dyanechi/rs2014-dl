use super::*;

#[derive(Clone)]
pub struct Mega {}

impl Driver for Mega {
    const DRIVER_NAME: &'static str = "Mega";
    const DOMAIN_URL: &'static str = "mega.nz";

    fn create() -> Self {
        Self {  }
    }
}
impl DownloadDriver for Mega {
    // type DlResult = ();
    fn download<'a>(&self, url: &'a str) -> ApiResult<Self::DlResult> {
        let parsed_url = Url::parse(&url).unwrap();
        
        let remote_host = remote_url(parsed_url)?;
        let download_path = PathBuf::from(Self::DOWNLOAD_DIR).join("");

        let bytes = Downloader::get(remote_host.as_str())?;

        let metadata = FileMeta::new()
            .with_id("")
            .with_title("")
            .build();

        Ok(DownloadedFile::new(metadata, bytes))
    }
}
// impl GetDownloader for Mega {
//     fn downloader(&self) -> Downloader<Box<Self>> {
//         Downloader::new(Box::new(self.clone()))
//     }
// }


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
// impl Driver<DlDriver<Mega>> for Mega {
//     fn create() -> DlDriver<Mega> {
//         DlDriver::new(Mega {  })
//     }
// }

// impl Driver<DlDriver<Mega>> for Mega {}
// impl DwDriver<Mega> for Mega {
//     fn downloader(&self) -> DlDriver<Mega> {
        
//     }
// }

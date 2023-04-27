use super::*;

#[derive(Clone)]
pub struct DropBox {}

impl Driver for DropBox {
    const DRIVER_NAME: &'static str = "DropBox";
    const DOMAIN_URL: &'static str = "www.dropbox.com";

    fn create() -> Self {
        Self {  }
    }
}
impl DownloadDriver for DropBox {
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

fn remote_url(url: Url) -> ApiResult<RemoteHost> {
    let mut url = url.clone();
    url.set_query(Some("dl=1"));

    let Some(mut segments) = url.path_segments() else {
        return Err(Error::RequestError(format!("uri path is missing")));
    };

    if let Some(location) = segments.next()
    && let Some(id) = segments.next()
    && let Some(file_name) = segments.next() {
        println!("Found dropbox file: '{file_name}' with id: '{id}'");
        if location != "s" {
            return Err(Error::RequestError(format!("uri path '{}' is invalid, expected '/s'", location)))
        }
        Ok(RemoteHost::new(url.as_str()))
    } else {
        Err(Error::RequestError(format!("URL '{}' is incorrect. It might be missing file uri to download.", url.to_string())))
    }
}
// impl Driver<DlDriver<DropBox>> for DropBox {
//     fn create() -> DlDriver<DropBox> {
//         DlDriver::new(DropBox {  })
//     }
// }
// impl Driver<DlDriver<DropBox>> for DropBox {}
// impl GetDownloader for DropBox {
//     fn downloader(&self) -> Downloader<Box<Self>> {
//         Downloader::new(Box::new(self.clone()))
//     }
//     // fn downloader(&self) -> DlDriver<DropBox> {
//     //     DlDriver::new(DropBox {})
//     // }
    
// }
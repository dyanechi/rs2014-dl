use super::*;

#[derive(Clone)]
pub struct DropBox {}

impl Driver for DropBox {
    const DOMAIN_URL: &'static str = "https://www.dropbox.com";
}
impl DownloadDriver for DropBox {
    type DlResult = ();
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
        where 
            U: Into<String>,
            F: Into<String>, {
        Err(Error::DownloadError("".to_string()))
    }
}
// impl Driver<DlDriver<DropBox>> for DropBox {
//     fn create() -> DlDriver<DropBox> {
//         DlDriver::new(DropBox {  })
//     }
// }
// impl Driver<DlDriver<DropBox>> for DropBox {}
impl GetDownloader for DropBox {
    fn downloader(&self) -> Downloader<Box<Self>> {
        Downloader::new(Box::new(self.clone()))
    }
    // fn downloader(&self) -> DlDriver<DropBox> {
    //     DlDriver::new(DropBox {})
    // }
    
}
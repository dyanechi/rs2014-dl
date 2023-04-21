use super::*;

#[derive(Clone)]
pub struct GDrive {}
impl DownloadDriver for GDrive {
    // const DL_DOMAIN: &'static str = "drive.google.com";
    type DlResult = ();
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
        where 
            U: Into<String>,
            F: Into<String>, {
        Err(Error::DownloadError("".to_string()))
    }
}
impl Downloader for GDrive {
    fn downloader(&self) -> DlDriver<Box<Self>> {
        DlDriver::new(Box::new(self.clone()))
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

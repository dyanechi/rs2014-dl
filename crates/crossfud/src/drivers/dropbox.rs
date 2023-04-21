use super::*;

#[derive(Clone)]
pub struct DropBox {}
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
impl Downloader for DropBox {
    fn downloader(&self) -> DlDriver<Box<Self>> {
        DlDriver::new(Box::new(self.clone()))
    }
    // fn downloader(&self) -> DlDriver<DropBox> {
    //     DlDriver::new(DropBox {})
    // }
    
}
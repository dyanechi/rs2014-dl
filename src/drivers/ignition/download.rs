use std::path::Path;

use super::*;


impl DownloadDriver for IgnitionDriver {
    const DOWNLOAD_DIR: &'static str = "./downloads";
    type DlResult = ();
    fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::DlResult>
        where 
            U: Into<String>,
            F: Into<String>, 
        {
            let file_path = Path::new(Self::DOWNLOAD_DIR).join(file_name.into());
            // if file_path.exists() { return Ok(DownloadState::Skipped ); }
            std::fs::create_dir_all(&file_path.parent().unwrap()).unwrap();
    
            // let download_url = self.parse_download_url(url)?;
            let download_url = url.into();
    
            // println!("[INFO] Downloading from '{download_url}'...");]
            match reqwest::blocking::get(&download_url) {
                Ok(resp) => {
                    let body = resp.bytes().unwrap();
                    if let Err(err) = std::fs::write(file_path, &body) {
                        return Err(Error::FileError(err.to_string()));
                    };
                },
                Err(error) => return Err(Error::RequestError(format!("{}", error.to_string())))
            }
    
            // Ok(DownloadState::Downloaded)
            
            Err(Error::DriverError(format!("Failed")))
    }
}
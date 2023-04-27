use std::str::FromStr;

use serde::{Serialize, Deserialize};

use super::*;

#[derive(Clone)]
pub struct GDrive {}

impl Driver for GDrive {
    const DRIVER_NAME: &'static str = "GDrive";
    const DOMAIN_URL: &'static str = "drive.google.com";

    fn create() -> Self {
        Self {  }
    }
}
impl DownloadDriver for GDrive {
    // type DlResult = &[u8];
    // type DlResult = ;
    fn download<'a>(&self, url: &'a str) -> ApiResult<Self::DlResult> {
        
        let file = Self::get_file_meta(url).expect("expected file");


        let download_url = file.download_url()?;
        // let download_path = PathBuf::from(Self::DOWNLOAD_DIR).join(&file.title);
        let bytes = Downloader::get(&download_url)?;

        Ok(DownloadedFile::new(file.into(), bytes))
    }
}

impl GDrive {
    fn get_file_meta(url: &str) -> ApiResult<GDriveFile> {
        let response = reqwest::blocking::get(url).unwrap();
        let body = response.text().unwrap();

        let re = regex::Regex::from_str(r"window\.viewerData = \{config: \{(.*),\},.*\}").expect("should build regex");
        let caps = re.captures(&body).expect("no matches found ");
        let json = caps.get(1).unwrap();
        let json_str = format!("{{{}}}", json.as_str().replace("'", &'"'.to_string())).replace(" ", "");
        let file_meta = serde_json::from_str::<GDriveFile>(&json_str).unwrap();
        // println!("{:#?}", file_meta);
        Ok(file_meta)

        // Err(Error::ApiError("()".to_string()))
    }
}


#[derive(Debug, Default, WithBuilder, Serialize, Deserialize)]
pub struct GDriveFile {
    id: String,
    title: String,

}
impl GDriveFile {
    fn download_url(&self) -> ApiResult<String> {
        if self.id.is_empty() {
            return Err(Error::DownloadError("Missing id in url".to_string()));
        }
        Ok(format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", self.id))
    }
    // fn download_url(&self, f: fn(file: &GDriveFile) -> ApiResult<String>) -> ApiResult<String> {
    //     f(self.clone())
    // }
}

impl Into<FileMeta> for GDriveFile {
    fn into(self) -> FileMeta {
        let GDriveFile { id, title, .. } = self;
        FileMeta { id, title }
    }
}



fn parse_url(url: Url) -> ApiResult<RemoteHost> {
    for (k, v) in url.query_pairs() {
        if k == "amp;resourcekey" {
            return Err(Error::DownloadError(format!("Link requires Google authorization: {}", url)))
        }
        if k == "id" {
            return Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", v)));
        }
    };
    
    let mut uri = url.path_segments().unwrap();
    if let Some(location) = uri.next()
    && let Some(d) = uri.next()
    && let Some(id) = uri.next()
    {
        if location != "file" || d != "d" {
            return Err(Error::DownloadError("Only file downloading is supported".to_string()));
        }
        return Ok(RemoteHost::new(&format!("https://drive.google.com/uc?id={}&export=download&confirm=yes", id)));
    } else {
        return Err(Error::DownloadError("Only file downloading is supported".to_string()));
    }

}

impl CacheDriver for GDrive {
    const CACHE_APP_NAME: &'static str = "GDrive";
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

use std::path::Path;

use reqwest::Url;

use crate::*;

// const DOWNLOADS_DIR: &'static str = "./downloads";


// pub enum DownloadState {
//     Downloaded,
//     Skipped,
// }

// type DownloadUrl = String;
// trait Download {
//     fn download(&self, url: impl Into<String>, file_name: impl Into<String>) -> ApiResult<DownloadState>;
//     fn parse_download_url(&self, url: impl Into<String>) -> ApiResult<DownloadUrl>;
//     fn verify_download_url(&self, url: impl Into<String>) -> ApiResult<()>;
// }


// impl Download for IgnitionDriver {
//     fn verify_download_url(&self, url: impl Into<String>) -> ApiResult<()> {
        
//         Ok(())
//     }
//     fn parse_download_url(&self, url: impl Into<String>) -> ApiResult<DownloadUrl> {
//         let parsed_url = Url::parse(&url.into()).expect("Failed to parse url");
//         let domain = parsed_url.domain().unwrap();
//         let download_url = match domain {
//             "drive.google.com" => {
//                 let mut id = format!("");
//                 for (k, v) in parsed_url.query_pairs(){
//                     if k == "id" { id = v.to_string(); break }
//                     if k == "amp;resourcekey" {
//                         return Err(Error::DownloadError(format!("Link requires Google authorization: {}", parsed_url)))
//                     }
//                 };
//                 if !id.is_empty() {
//                     format!("https://drive.google.com/uc?id={}&export=download", id)
//                 } else {
//                     let mut uri = parsed_url.path_segments().unwrap();
//                     uri.next();
//                     if let Some(sub_location) = uri.next() {
//                         if sub_location == "folders" {
//                             return Err(Error::DownloadError("Downloading from folders not supported".to_string()));
//                         }
//                         // return Err(Error::DownloadError("Should have sub-location".to_string()));
//                     };
                    
//                     let Some(id) = uri.next() else {
//                         return Err(Error::DownloadError("Can't find id in url".to_string()));
//                     };
//                     format!("https://drive.google.com/uc?id={}&export=download", id)
//                 }
//             },
//             "mega.nz" => {
//                 return Err(Error::DownloadError(format!("Domain not supported: '{domain}'")) )
//             },
//             "www.dropbox.com" => {
//                 let mut url = parsed_url.clone();
//                 url.set_query(Some("dl=1"));
//                 url.to_string()
//             },
//             _ => return Err(Error::DownloadError(format!("Unrecognized domain: '{domain}'")) )
//         };

//         Ok(download_url)
//     }
//     fn download(&self, url: impl Into<String>, file_name: impl Into<String>) -> ApiResult<DownloadState> {
//         let file_path = Path::new(DOWNLOADS_DIR).join(file_name.into());
//         if file_path.exists() { return Ok(DownloadState::Skipped ); }
//         std::fs::create_dir_all(&file_path.parent().unwrap()).unwrap();

//         // let download_url = self.parse_download_url(url)?;
//         let download_url = url.into();

//         // println!("[INFO] Downloading from '{download_url}'...");]
//         match reqwest::blocking::get(&download_url) {
//             Ok(resp) => {
//                 let body = resp.bytes().unwrap();
//                 if let Err(err) = std::fs::write(file_path, &body) {
//                     return Err(Error::FileError(err.to_string()));
//                 };
//             },
//             Err(error) => return Err(Error::RequestError(format!("{}", error.to_string())))
//         }

//         Ok(DownloadState::Downloaded)
//     }
// }
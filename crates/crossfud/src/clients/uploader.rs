

use super::*;

pub enum UpRemoteHost {
    GDrive,
    DropBox,
    Custom(RemoteHost),
    Unknown,
}

pub struct Uploader {
    // driver: ,
    remote: RemoteHost,
    url: RemoteHost,
}

impl Client for Uploader {
    const CLIENT_NAME: &'static str = "Crossfud Uploader";
}
impl UploadClient for Uploader {
    
}
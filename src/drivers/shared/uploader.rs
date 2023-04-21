

use super::*;
pub trait UploadDriver {}

pub enum UpRemoteHost {
    GDrive,
    DropBox,
    Custom(RemoteHost),
    Unknown,
}

pub struct Uploader<T: UploadDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteUrl,
}


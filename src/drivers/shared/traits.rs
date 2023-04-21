use super::*;

pub trait AsAny { fn as_any(&self) -> &dyn Any; }
pub trait ProcessDriver: Any + AsAny + DownloadDriver + UploadDriver + FetchDriver {}

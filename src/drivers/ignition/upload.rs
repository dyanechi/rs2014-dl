use super::*;

impl UploadDriver for IgnitionDriver {
    type UpResult = ();
    fn upload(&self) -> ApiResult<Self::UpResult> {
        Err(Error::DriverError("".to_string()))
    }
}
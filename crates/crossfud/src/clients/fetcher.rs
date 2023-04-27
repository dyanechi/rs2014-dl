use super::*;


pub struct Fetcher {
    remote: RemoteHost,
    url: RemoteHost,
}

impl Client for Fetcher {
    const CLIENT_NAME: &'static str = "Crossfud Fetcher";
}
impl FetchClient for Fetcher {
    fn fetch<T>(&self) -> ApiResult<T> {
        Err(Error::ApiError("".to_string()))
    }
}

// impl<T: FetchDriver> Driver for Fetcher<T> {
//     const DRIVER_NAME: &'static str = "";
//     const DOMAIN_URL: &'static str = T::DOMAIN_URL;
// }
// impl<T: FetchDriver> FetchDriver for Fetcher<T> {
//     const FT_URL: &'static str = T::FT_URL;
//     fn fetch<R: DeserializeOwned>(&self) -> ApiResult<R> {
//         self.driver.fetch()
//     }
// }

// impl Driver for Fetcher {
//     const DRIVER_NAME: &'static str = "";
//     const DOMAIN_URL: &'static str = "";
// }
// impl FetchDriver for Fetcher {
//     const FT_URL: &'static str = "";
//     fn fetch<R: DeserializeOwned>(&self) -> ApiResult<R> {
//         self.driver.fetch()
//     }
// }
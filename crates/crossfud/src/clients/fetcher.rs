use super::*;


pub struct Fetcher<T: FetchDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteHost,
}

impl<T: FetchDriver> Driver for Fetcher<T> {
    const DOMAIN_URL: &'static str = T::DOMAIN_URL;
}
impl<T: FetchDriver> FetchDriver for Fetcher<T> {
    const FT_URL: &'static str = T::FT_URL;
    fn fetch<R: DeserializeOwned>(&self) -> ApiResult<R> {
        self.driver.fetch()
    }
}

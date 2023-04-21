use super::*;
pub trait FetchDriver { }

pub struct Fetcher<T: FetchDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteUrl,
}


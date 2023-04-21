use super::*;


pub struct Fetcher<T: FetchDriver> {
    driver: T,
    remote: RemoteHost,
    url: RemoteUrl,
}


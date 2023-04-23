use super::*;


pub struct Cacher<T: CacheDriver> {
    driver: T,
    remote: RemoteHost,
    // url: ReemoteUrl,
}


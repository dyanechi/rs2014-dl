use super::*;


pub struct Cacher {
    // driver: T,
    remote: RemoteHost,
    // url: ReemoteUrl,
}

impl Client for Cacher {
    const CLIENT_NAME: &'static str = "Crossfud Cacher";
}
impl CacheClient for Cacher {

}

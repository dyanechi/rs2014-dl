use crossfud::CacheDriver;

use crate::IgnitionDriver;



impl CacheDriver for IgnitionDriver {
    const CACHE_APP_NAME: &'static str = "ignition";
}
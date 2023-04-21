use std::any::Any;
use reqwest::Url;

use crate::*;

mod shared;
use shared::*;

mod ignition;
pub use ignition::*;

pub struct DriverInstance {
    driver: Option<Box<dyn ProcessDriver>>,
    remote: RemoteHost,
    url: RemoteUrl,
}

impl DriverInstance {
    pub fn create(driver: Driver) -> ApiResult<Self> {
        Ok(Self {
            driver: match driver {
                Driver::Ignition => Some(Box::new(IgnitionDriver::from_env())),
                _ => None,
                // Driver::Corosion => Box::new(CorosionDriver::new()),
                // Driver::Unknown => return Err(Error::DriverError("No driver provider".to_string()))
            },
            remote: Default::default(),
            url: Default::default(),
        })
    }

    pub fn driver_ref<T: 'static>(&self) -> Option<&T> {
        self.driver.and_then(|driver| driver.as_any().downcast_ref::<T>())
    }
    pub fn driver_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.driver.and_then(|driver| driver.as_any().downcast_mut::<T>())
    }
}


#[derive(Default, Clone)]
pub enum Driver {
    #[default]
    Ignition,
    Unknown,
}
impl Driver {
    pub fn new(self) -> ApiResult<DriverInstance> {
        DriverInstance::create(self)
    }
}

// impl DriverInstance {
//     pub fn ignition(self) -> Option<Box<IgnitionDriver>> {
//         match self {
//             Self::Ignition(driver) => Some(driver.driver),
//             _ => None
//         }
//     }

//     // pub fn driver(&self) -> Option<RequestDriver<dyn ProcessDriver>> {
//     //     None
//     // }


//     // pub fn unbox<T: ProcessDriver>(self) -> Option<T> {
//     //     let d = match self {
//     //         DriverInstance::Ignition(driver) => Some(driver),
//     //         DriverInstance::Unknown(driver) => Some(driver)
//     //     };
//     // }
// }




// impl Driver {
//     pub fn ignition(&mut self) -> Option<&mut RequestDriver<IgnitionDriver>> {
//         match self {
//             Self::Ignition => Some(&mut RequestDriver::new(IgnitionDriver::new())),
//             _ => None
//         }
//     }
// }


pub fn test() {
    let driver = Driver::Ignition.new().unwrap();
    let driver = driver.driver_ref::<IgnitionDriver>().unwrap();
    // let DriverInstance::Ignition(mut driver) = Driver::Ignition.new() else {panic!()};
    // let driver = Driver::Ignition.new().driver().unwrap();
    // driver.driver.;
    // let fetcher = driver.driver.fetcher();
    // let dri = RequestDriver::new(IgnitionDriver::new());
    // let d = driver.driver();
}


#[derive(Debug, Default, Clone)]
struct RemoteUrl {
    url: String,
    parsed: bool,
}
impl RemoteUrl {
    fn new(url: &str) -> RemoteUrl {
        RemoteUrl { url: url.into(), parsed: false }
    }

    fn parse(self) -> Self {

        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct RemoteHost {
    url: RemoteUrl,
}
impl RemoteHost {
    // pub fn url(&self, id: impl Into<String>) -> String {
    //     match self {
    //         Self::Google => format!("https://drive.google.com/uc?id={}&export=download", id.into()),
    //         Self::DropBox => format!(""),
    //         Self::Mega => format!(""),
    //         Self::Other(s) => format!("{s}"),
    //         Self::Unknown => format!("")
    //     }.to_string()
    // }

    // pub fn from_url(url: impl Into<String>) -> ApiResult<Self> {
    //     let parsed_url = Url::parse(&url.into()).expect("Failed to parse url");

    //     match parsed_url.domain().unwrap() {
    //         "" => Ok(Self::Unknown),
    //         _ => Ok(Self::Unknown)
    //     }

    // }
}
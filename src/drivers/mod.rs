use std::any::Any;
use reqwest::Url;

use crate::*;

pub mod shared;
use shared::*;
pub use shared::traits::*;

mod ignition;
pub use ignition::*;

pub struct DriverInstance<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> {
    driver: Option<RequestDriver<T>>,
    remote: RemoteHost,
    url: RemoteUrl,
}


impl<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> DriverInstance<T> {

    pub fn new(driver: T) -> Self {
        Self {
            driver: Some(Box::new(driver)),
            remote: Default::default(),
            url: Default::default(),
        }
    }

    // fn create(driver: Driver) -> ApiResult<Self> {
    //     Ok(Self {
    //         driver: match driver {
    //             Driver::Ignition => Some(Box::new(IgnitionDriver::from_env())),
    //             // Driver::Mistery => Some(Box::new(MisteryDriver {})),
    //             Driver::Unknown => return Err(Error::DriverError("No driver provider".to_string())),
    //             _ => None,
    //         },
    //         remote: Default::default(),
    //         url: Default::default(),
    //     })
    // }

    pub fn driver_ref(&self) -> Option<&T> {
        self.driver.as_ref().and_then(|driver| driver.as_any().downcast_ref::<T>())
    }
    pub fn driver_mut(&mut self) -> Option<&mut T> {
        self.driver.as_deref_mut().and_then(|driver| driver.as_any_mut().downcast_mut::<T>())
    }
    // pub fn driver_mut<T: 'static>(&mut self) -> Option<&mut T> {
    //     let Some(driver) = self.driver else { return None; };
    //     driver.as_any().downcast_mut::<T>()
    // }
}

// impl<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> DownloadDriver for DriverInstance<T> {
//     const DOWNLOAD_DIR: &'static str = "./downloads";
//     type Output = ();
//     fn download<U, F>(&self, url: U, file_name: F)-> ApiResult<Self::Output>
//         where 
//             U: Into<String>,
//             F: Into<String>, {
//         match self.driver.unwrap() {
//             Some(driver) => {
//                 driver.download(url, file_name)
//             }
//             None => Err(Error::DriverError("No driver specified".to_string())),
//         }
//     }
// }



struct MisteryDriver {}
impl AsAny for MisteryDriver {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
// impl DownloadDriver for MisteryDriver {}
// impl UploadDriver for MisteryDriver {}
// impl FetchDriver for MisteryDriver {}
// impl ProcessDriver for MisteryDriver {}


#[derive(Default, Clone)]
pub enum Driver {
    #[default]
    Ignition,
    Mistery,
    Unknown,
}
impl Driver {
    // pub fn new<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver>(self) -> ApiResult<DriverInstance<T>> {
    //     DriverInstance::create(self)
    // }
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
    let driver = IgnitionDriver::create();

    // let driver = Driver::Ignition.new().unwrap();
    // let driver = driver.driver_ref::<IgnitionDriver>().unwrap();

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
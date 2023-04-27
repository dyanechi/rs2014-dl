use std::{any::Any, ops::{Deref, DerefMut}, borrow::BorrowMut};

use crate::*;
use std::path::{Path, PathBuf};

pub mod traits;
use serde::{Serialize, Deserialize};
pub use traits::*;
// use shared::*;
// pub use shared::traits::*;

pub mod gdrive;
pub mod dropbox;
pub mod mega;
pub mod custom;
pub use gdrive::*;
pub use dropbox::*;
pub use mega::*;
pub use custom::*;

#[derive(Debug, Clone)]
pub struct DriverMeta {
    remote_host: RemoteHost
}

pub enum DriverKind {
    GDrive(GDrive),
    DropBox(DropBox),
    Mega(Mega),
    // Custom(DlDriver<Custom>),
    // Unknown,
}
impl DriverKind {
    pub fn from_url(url: &str) -> ApiResult<Self> {
        let parsed_url = Url::parse(url).expect("should be valid URL");
        let domain = parsed_url.domain().expect("expected valid domain in URL");
        match domain {
            GDrive::DOMAIN_URL => Ok(Self::GDrive(GDrive {  })),
            DropBox::DOMAIN_URL => Ok(Self::DropBox(DropBox {  })),
            Mega::DOMAIN_URL => Ok(Self::Mega(Mega {  })),
            _ => Err(Error::DriverError(format!("Domain '{domain}' is not supported"))),
        }
    }
}

#[derive(Debug, Default, Clone, WithBuilder, Serialize, Deserialize)]
pub struct FileMeta {
    pub id: String,
    pub title: String,
}


// pub struct ClientInstance<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> {
//     driver: Option<RequestDriver<T>>,
//     remote: RemoteHost,
//     url: RemoteUrl,
// }

// impl<T: ProcessDriver + DownloadDriver + UploadDriver + FetchDriver> ClientInstance<T> {

//     pub fn new(driver: T) -> Self {
//         Self {
//             driver: Some(Box::new(driver)),
//             remote: Default::default(),
//             url: Default::default(),
//         }
//     }

//     pub fn driver_ref(&self) -> Option<&T> {
//         self.driver.as_ref().and_then(|driver| driver.as_any().downcast_ref::<T>())
//     }
//     pub fn driver_mut(&mut self) -> Option<&mut T> {
//         self.driver.as_deref_mut().and_then(|driver| driver.as_any_mut().downcast_mut::<T>())
//     }
// }



pub fn test() {
    // let driver = IgnitionDriver::create();

    // let driver = Driver::Ignition.new().unwrap();
    // let driver = driver.driver_ref::<IgnitionDriver>().unwrap();

    // let DriverInstance::Ignition(mut driver) = Driver::Ignition.new() else {panic!()};
    // let driver = Driver::Ignition.new().driver().unwrap();
    // driver.driver.;
    // let fetcher = driver.driver.fetcher();
    // let dri = RequestDriver::new(IgnitionDriver::new());
    // let d = driver.driver();
}


// #[derive(Debug, Default, Clone)]
// pub struct RemoteUrl {
//     url: String,
//     parsed: bool,
// }
// impl RemoteUrl {
//     pub fn new(url: &str) -> RemoteUrl {
//         RemoteUrl { url: url.into(), parsed: false }
//     }

//     fn parse(self) -> Self { self }
// }

#[derive(Debug, Clone)]
pub struct RemoteHost(Url);
impl Deref for RemoteHost {
    type Target = Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RemoteHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.borrow_mut()
    }
}
impl RemoteHost {
    pub fn new(url: &str) -> RemoteHost {
        let parsed_url = Url::parse(&format!("{url}"))
            .expect(&format!("failed to parse url from domain '{}'", url));

        Self(parsed_url)
        // parsed_url.
        // Self {
        //     url: url.to_string(),
        //     domain: match parsed_url.domain() {
        //         Some(domain) => Some(domain.to_string()),
        //         None => None
        //     }
        // }
    }
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
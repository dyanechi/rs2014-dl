use reqwest::Url;

use crate::*;



struct DlUrl {
    url: String,
    parsed: bool,
}
impl DlUrl {
    fn new(url: &str) -> DlUrl {
        DlUrl { url: url.into(), parsed: false }
    }

    fn parse(self) -> Self {

        self
    }
}

pub struct Downloader {
    driver: DlDriver,
    remote: RemoteHost,
    url: DlUrl,
}

pub trait DownloadDriver {

}

pub enum DlDriver {
    Ignition(Ignition),
}

pub enum RemoteHost {
    Google,
    DropBox,
    Mega,
    Other(String),
    Unknown,
}
impl RemoteHost {
    pub fn url(&self, id: impl Into<String>) -> String {
        match self {
            Self::Google => format!("https://drive.google.com/uc?id={}&export=download", id.into()),
            Self::DropBox => format!(""),
            Self::Mega => format!(""),
            Self::Other(s) => format!("{s}"),
            Self::Unknown => format!("")
        }.to_string()
    }

    pub fn from_url(url: impl Into<String>) -> Self {
        let parsed_url = Url::parse(&url.into()).expect("Failed to parse url");

        match parsed_url.domain().unwrap() {
            "" => Self::Unknown,
            _ => Self::Unknown
        }

    }
}
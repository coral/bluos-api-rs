use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),

    #[error("Could not find BluOS controller")]
    NoBluOSError,

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    XMLError(#[from] serde_xml_rs::Error),

    #[error("IDK BRO")]
    Unknown,
}

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

    #[error(transparent)]
    CancelError(#[from] std::sync::mpsc::SendError<bool>),

    #[error("Already discovering using zeroconf")]
    AlreadyDiscovering,

    #[error("IDK BRO")]
    Unknown,
}

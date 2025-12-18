use log::SetLoggerError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to parse input version.")]
    VersionError(#[from] semver::Error),

    #[error("Invalid tag name {0}")]
    InvalidTagError(String),

    #[error("Not on main branch ({0}).")]
    NotOnMainBranchError(String),

    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    #[error("UTF8 Error")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("Failed to initialize logger")]
    LoggerInitError(#[from] SetLoggerError),

    #[error("New git tag and cargo package version do not match")]
    VersionMismatchError(String),

    #[error("")]
    ParseError(String),
}

use thiserror::Error;

/// Error type for handling Docker image related failures
#[derive(Error, Debug)]
pub enum Error {
    /// Error caused by missing manifest file
    #[error("manifest is missing from docker image")]
    MissingImageManifest,

    /// Error caused by invalid content of manifest file
    #[error("invalid content in manifest file")]
    InvalidImageManifest,

    /// Error caused by missing configuration file
    #[error("manifest is missing from docker image")]
    MissingImageConfiguration,

    /// Error caused by invalid content of configuration file
    #[error("invalid content in manifest file")]
    InvalidImageConfiguration,
}

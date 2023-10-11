use thiserror::Error;

/// Error type for handling Docker repositories related failures
#[derive(Error, Debug)]
pub enum Error {
    /// Error caused by missing repositories file
    #[error("repositories file is missing")]
    MissingRepositories,

    /// Error caused by invalid content of repositories file
    #[error("repositories file is missing")]
    InvalidRepositories,
}

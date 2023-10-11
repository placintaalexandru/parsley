use crate::docker;
use std::io;
use thiserror::Error;

pub type ParsleyResult<T> = Result<T, ParsleyError>;

/// Error types for handling different sources of failure
#[derive(Error, Debug)]
pub enum ParsleyError {
    /// Custom error that cannot be mapped to something specific
    #[error("{0}")]
    Other(String),

    /// Error caused by an IO operation
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// Error caused by a serialization / deserialization operation
    #[error("serde error: {0}")]
    SerDe(#[from] serde_json::Error),

    /// Error caused by builders
    #[error("uninitialized field: {0}")]
    Builder(#[from] derive_builder::UninitializedFieldError),

    /// Errors caused by OCI spec
    #[error("oci spec error: {0}")]
    OCI(#[from] oci_spec::OciSpecError),

    /// Error caused by Docker image
    #[error("docker image error: {0}")]
    Docker(#[from] docker::error::Error),
}

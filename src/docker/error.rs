use crate::docker::distribution;
use crate::docker::image;
use thiserror::Error;

/// Error type for handling Docker related failures
#[derive(Error, Debug)]
pub enum Error {
    #[error("docker image error: {0}")]
    ImageError(image::error::Error),

    #[error("docker distribution error: {0}")]
    DistributionError(distribution::error::Error),
}

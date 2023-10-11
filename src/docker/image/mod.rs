//! [Docker Image Specification](https://github.com/moby/moby/blob/master/image/spec/spec.md) types
//! and definitions.

mod config;
pub(crate) mod error;
pub(crate) mod manifest;

pub use config::*;
pub use manifest::*;

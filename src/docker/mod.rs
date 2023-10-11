pub mod distribution;
pub(crate) mod error;
pub mod image;

pub use error::*;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    /// Creates the path to the directory containing Docker test data
    pub(crate) fn test_data_path<P>(path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/docker")
            .join(path)
    }
}

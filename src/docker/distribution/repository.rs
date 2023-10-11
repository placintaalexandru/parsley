use crate::error::{ParsleyError, ParsleyResult};
use crate::util;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

/// Map from image tag to layer hash.
///
/// Implemented as a tuple struct in order to implement foreign traits on the type, impossible if
/// defined as a type alias.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Repository(BTreeMap<String, String>);

/// Map from image name to different tags.
///
/// Implemented as a tuple struct in order to implement foreign traits on the type, impossible if
/// defined as a type alias.
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Repositories(BTreeMap<String, Repository>);

impl FromStr for Repositories {
    type Err = ParsleyError;

    /// Attempts to load repositories data from a JSON string.
    ///
    /// # Errors
    ///
    /// [ParsleyError::SerDe](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use std::str::FromStr;
    /// use parsley::docker::distribution;
    ///
    /// let s = "";
    /// let repositories = distribution::Repositories::from_str(&s).unwrap();
    /// ```
    fn from_str(s: &str) -> ParsleyResult<Self> {
        util::json::from_str(s)
    }
}

impl Repositories {
    /// Attempts to load repositories data from a file.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::Io) if the file does not exist
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::distribution;
    ///
    /// let repositories = distribution::Repositories::from_file("repositories").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> ParsleyResult<Self> {
        util::json::from_file(path)
    }

    /// Attempts to load repositories data from bytes of JSON text.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::distribution;
    ///
    /// let bytes = vec![];
    /// let repositories = distribution::Repositories::from_slice(&bytes).unwrap();
    /// ```
    pub fn from_slice(v: &[u8]) -> ParsleyResult<Self> {
        util::json::from_slice(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docker;

    #[test]
    fn deserialize() {
        let path = docker::tests::test_data_path("repositories.json");
        Repositories::from_file(path).expect("Could not deserialize from file {path}");
    }

    #[test]
    fn serde() {
        let path = docker::tests::test_data_path("repositories.json");
        let deserialized_repositories =
            Repositories::from_file(path).expect("Could not deserialize from file {path}");
        let serialized_repositories =
            serde_json::to_string(&deserialized_repositories).expect("Failed to serialize");
        let re_deserialized_repositories = Repositories::from_str(&serialized_repositories)
            .expect("Could not deserialize from serialization");

        assert_eq!(
            deserialized_repositories, re_deserialized_repositories,
            "Deserialized repositories from serialized repositories is different"
        );
    }
}

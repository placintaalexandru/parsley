use crate::error::{ParsleyError, ParsleyResult};
use crate::util;
use derive_builder::Builder;
use getset::Getters;
use oci_spec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

/// An image manifest item provides details about the image: where to find within the artifact the
/// configuration file, set of layers used etc.
///
/// # Example
/// ```
/// use std::collections::BTreeMap;
/// use parsley::docker::image::ManifestItemBuilder;
///
/// let manifest_item = ManifestItemBuilder::default()
///     .config(String::default())
///     .repo_tags(Vec::default())
///     .layers(Vec::default())
///     .parent(String::default())
///     .layer_sources(BTreeMap::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Getters, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "ParsleyError")
)]
#[getset(get = "pub")]
pub struct ManifestItem {
    config: String,
    repo_tags: Vec<String>,
    layers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    layer_sources: Option<BTreeMap<String, oci_spec::image::Descriptor>>,
}

/// The `manifest.json` file provides the image JSON for the top-level image and, optionally, for
/// parent images that this image was derived from.
///
/// It consists of an array of metadata entries, defined by [ManifestItem](ManifestItem).
///
/// # Example
/// ```
/// use parsley::docker::image::ImageManifest;
/// use parsley::docker::image::ManifestItemBuilder;
///
/// let image_manifest = ImageManifest(vec![]);
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImageManifest(pub Vec<ManifestItem>);

impl FromStr for ImageManifest {
    type Err = ParsleyError;

    /// Attempts to load an image manifest from a JSON string.
    ///
    /// # Errors
    ///
    /// [ParsleyError::SerDe](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use std::str::FromStr;
    /// use parsley::docker::image;
    ///
    /// let s = "";
    /// let image_manifest = image::ImageManifest::from_str(&s).unwrap();
    /// ```
    fn from_str(s: &str) -> ParsleyResult<Self> {
        util::json::from_str(s)
    }
}

impl ImageManifest {
    /// Attempts to load an image manifest from a file.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::Io) if the file does not exist
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::image;
    ///
    /// let image_manifest = image::ImageManifest::from_file("manifest.json").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> ParsleyResult<Self> {
        util::json::from_file(path).map(Self)
    }

    /// Attempts to load an image manifest from bytes of JSON text.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::image;
    ///
    /// let bytes = vec![];
    /// let image_manifest = image::ImageManifest::from_slice(&bytes).unwrap();
    /// ```
    pub fn from_slice(v: &[u8]) -> ParsleyResult<Self> {
        util::json::from_slice(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docker;

    fn manifest() -> ImageManifest {
        ImageManifest(vec![ManifestItemBuilder::default()
            .config(
                "ee56d70bcdf1aeca472a9899de653eb4d72f4a3ac31d9b0b95e677488ce766f3.json".to_owned(),
            )
            .repo_tags(vec!["postgres:15.4".to_owned()])
            .layers(vec![
                "3b05311756d94678c1ea8e45bf7665a4e29f850c31c6f58d6c28403c6fdc0cdc/layer.tar"
                    .to_owned(),
                "454d82adf13f02e53baeae05d06b595b34bbab2836977c6b679488ec038449c3/layer.tar"
                    .to_owned(),
                "c039956656e1c9cd1e2d72dba02179b8d9008e0c0771af344944e218c7dc3351/layer.tar"
                    .to_owned(),
            ])
            .build()
            .expect("Manifest Build Item 1")])
    }

    #[test]
    fn deserialize() {
        let manifest_path = docker::tests::test_data_path("manifest.json");
        let deserialized_manifest =
            ImageManifest::from_file(manifest_path).expect("Could not deserialize from file");

        assert_eq!(
            deserialized_manifest,
            manifest(),
            "Deserialized manifest does not match expected one"
        )
    }

    #[test]
    fn serde() {
        let manifest_path = docker::tests::test_data_path("manifest.json");
        let deserialized_manifest =
            ImageManifest::from_file(manifest_path).expect("Could not deserialize from file");
        let serialized_manifest =
            serde_json::to_string(&deserialized_manifest).expect("Failed to serialize");
        let re_deserialized_manifest = ImageManifest::from_str(&serialized_manifest)
            .expect("Could not deserialize from serialization");

        assert_eq!(
            re_deserialized_manifest, deserialized_manifest,
            "Deserialized manifest from serialized manifest is different"
        )
    }
}

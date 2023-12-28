use crate::error::{ParsleyError, ParsleyResult};

use crate::util;
use derive_builder::Builder;
use getset::Getters;
use oci_spec;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

/// Docker OCI image configuration according.
/// The image is composed by a base set of specifications that comply with the OCI specifications
/// and a Docker specific extension.
///
/// For more details refer to [Go Spec](https://github.com/moby/moby/blob/master/image/spec/specs-go/v1/image.go#L20C14-L20C14).
///
/// # Example
/// ```
/// use parsley::docker::image;
/// use oci_spec::image as oci_image;
///
/// let image_config = image::ImageConfigurationBuilder::default()
///     .oci_spec(oci_image::ImageConfiguration::default())
///     .docker_oci_extension(image::ImageConfigurationExtension::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Getters, Clone, Debug, Default, Eq, PartialEq)]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "ParsleyError")
)]
#[getset(get = "pub")]
pub struct ImageConfiguration {
    /// Standard OCI specifications.
    oci_spec: oci_spec::image::ImageConfiguration,

    /// Docker specific extension of the OCI specifications.
    docker_oci_extension: Option<ImageConfigurationExtension>,
}

/// Custom serialization implementation since, both OCI specification and Docker extension
/// fields are required to be merged under the same field (e.g. `config` field of the image
/// specification).
impl Serialize for ImageConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut merged_config = serde_json::to_value(&self.oci_spec)
            .map_err(|err| <S::Error as serde::ser::Error>::custom(err.to_string()))?;
        let docker_extension = serde_json::to_value(&self.docker_oci_extension)
            .map_err(|err| <S::Error as serde::ser::Error>::custom(err.to_string()))?;

        util::json::merge(&mut merged_config, docker_extension);

        merged_config.serialize(serializer)
    }
}

/// Custom deserialization implementation since, both OCI specification and Docker extension
/// fields are required to be extracted from the same field (e.g. `config` field of the image
/// specification).
impl<'de> Deserialize<'de> for ImageConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Retrieve the JSON, as the 'config' field is used by both OCI spec and Docker extensions
        let full_json: serde_json::Value = Deserialize::deserialize(deserializer)?;

        // Deserialize the JSON twice: once for OCI spec and once for Docker extensions
        let oci_spec = Deserialize::deserialize(full_json.clone())
            .map_err(|json_err| serde::de::Error::custom(json_err.to_string()))?;
        let docker_oci_extension = Deserialize::deserialize(full_json)
            .map_err(|json_err| serde::de::Error::custom(json_err.to_string()))?;

        Ok(Self {
            docker_oci_extension,
            oci_spec,
        })
    }
}

/// [Docker extension](https://github.com/moby/moby/blob/master/image/spec/specs-go/v1/image.go#L23)
/// that covers different information that Docker adds on top of the OCI specifications.
///
/// Every field in the structure corresponds to extra information Docker adds to every field on top
/// of standard the OCI specification.
///
/// # Example
/// ```
/// use parsley::docker::image;
/// use parsley::docker::image::ConfigExtension;
///
/// let extension = image::ImageConfigurationExtensionBuilder::default()
///     .config(ConfigExtension::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Getters, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "ParsleyError")
)]
#[getset(get = "pub")]
pub struct ImageConfigurationExtension {
    /// Extra fields in the `config` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    config: Option<ConfigExtension>,
}

/// Covers all extra fields that Docker adds in `config` field of the OCI image specifications.
///
/// # Example
/// ```
/// use parsley::docker::image;
/// use parsley::docker::image::HealthcheckConfig;
///
/// let config_extension = image::ConfigExtensionBuilder::default()
///     .memory(u64::default())
///     .memory_swap(u64::default())
///     .cpu_shares(u16::default())
///     .args_escaped(bool::default())
///     .health_check(HealthcheckConfig::default())
///     .on_build(Vec::default())
///     .shell(Vec::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Getters, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "ParsleyError")
)]
#[serde(rename_all = "PascalCase")]
#[getset(get = "pub")]
pub struct ConfigExtension {
    /// Memory limit (in bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    memory: Option<u64>,
    /// Total memory usage (memory + swap).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    memory_swap: Option<u64>,
    /// CPU shares (relative weight vs. other containers).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cpu_shares: Option<u16>,
    /// Used for Windows images to indicate that the Entrypoint or Cmd or both, contain only a
    /// single element array that is a pre-escaped, and combined into a single string,
    /// **CommandLine**.
    ///
    /// If "true", the value in Entrypoint or CmdCmd should be used as-is to avoid double escaping.
    #[serde(default)]
    args_escaped: bool,
    /// Test to perform to determine whether the container is healthy. Here is an example:
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        // Healthcheck does not respect the naming pattern, thus the alias
        alias = "Healthcheck"
    )]
    health_check: Option<HealthcheckConfig>,
    /// Defines "trigger" instructions to be executed at a later time, when the image is used as the
    /// base for another build.
    ///
    /// Each trigger will be executed in the context of the downstream build, as if it had been
    /// inserted immediately after the *FROM* instruction in the downstream Dockerfile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    on_build: Option<Vec<String>>,
    /// Override the default shell used for the *shell* form of commands during "build".
    ///
    /// The default shell on Linux is `["/bin/sh", "-c"]`, and `["cmd", "/S", "/C"]` on Windows.
    ///
    /// This field is set by the SHELL instruction in a Dockerfile, and *must* be written in JSON form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shell: Option<Vec<String>>,
}

impl FromStr for ImageConfiguration {
    type Err = ParsleyError;

    /// Attempts to load an image configuration from a JSON string.
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
    /// let image_config = image::ImageConfiguration::from_str(&s).unwrap();
    /// ```
    fn from_str(s: &str) -> ParsleyResult<Self> {
        util::json::from_str(s)
    }
}

impl ImageConfiguration {
    /// Attempts to load an image configuration from a file.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::Io) if the file does not exist
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::image;
    ///
    /// let image_config = image::ImageConfiguration::from_file("1bc9978a2dd04fb656d9055670b5beee1c948ca3b65cade7783c2d3bab306141.json").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> ParsleyResult<Self> {
        util::json::from_file(path)
    }

    /// Attempts to load an image configuration from bytes of JSON text.
    ///
    /// # Errors
    /// [ParsleyError::Io](ParsleyError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    /// ``` no_run
    /// use parsley::docker::image;
    ///
    /// let bytes = vec![];
    /// let image_config = image::ImageConfiguration::from_slice(&bytes).unwrap();
    /// ```
    pub fn from_slice(v: &[u8]) -> ParsleyResult<Self> {
        util::json::from_slice(v)
    }
}

/// HealthcheckConfig holds configuration settings for the HEALTHCHECK feature.
///
/// For more details refer to the [Go Spec](https://github.com/moby/moby/blob/f6fa56194f1c9cfd2e4ae41a17f75ab6b04f82df/image/spec/specs-go/v1/image.go#L35).
///
/// # Example
/// ```
/// use std::time::Duration;
/// use parsley::docker::image;
///
/// let check = image::HealthcheckConfigBuilder::default()
///     .test(Vec::default())
///     .interval(Duration::default())
///     .timeout(Duration::default())
///     .start_interval(Duration::default())
///     .retries(u32::default())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "ParsleyError")
)]
pub struct HealthcheckConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    test: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "util::json::serialize_duration",
        deserialize_with = "util::json::deserialize_duration"
    )]
    interval: Option<Duration>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "util::json::serialize_duration",
        deserialize_with = "util::json::deserialize_duration"
    )]
    timeout: Option<Duration>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "util::json::serialize_duration",
        deserialize_with = "util::json::deserialize_duration"
    )]
    start_interval: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docker;
    use oci_spec::image;
    use std::collections::HashMap;

    fn config() -> ImageConfiguration {
        let docker_oci_extension = ImageConfigurationExtensionBuilder::default()
            .config(
                ConfigExtensionBuilder::default()
                    .memory(2048_u64)
                    .memory_swap(4096_u64)
                    .cpu_shares(8_u16)
                    .args_escaped(false)
                    .shell(vec![
                        "/bin/bash".to_owned(),
                        "-o".to_owned(),
                        "pipefail".to_owned(),
                        "-c".to_owned(),
                    ])
                    .on_build(vec!["a".to_owned(), "b".to_owned()])
                    .health_check(
                        HealthcheckConfigBuilder::default()
                            .test(vec![
                                "CMD-SHELL".to_owned(),
                                "/usr/bin/check-health localhost".to_owned(),
                            ])
                            .interval(Duration::from_nanos(30000000000))
                            .timeout(Duration::from_nanos(10000000000))
                            .start_interval(Duration::from_nanos(3000000000))
                            .retries(3_u32)
                            .build()
                            .expect("Build Docker OCI Extension: Healthcheck"),
                    )
                    .build()
                    .expect("Build Docker Config Extension"),
            )
            .build()
            .expect("Docker OCI Image Extension");
        let oci_spec = image::ImageConfigurationBuilder::default()
            .created("2023-08-16T06:40:57.929475525Z")
            .author("author".to_owned())
            .architecture(image::Arch::ARM64)
            .os(image::Os::Linux)
            .config(image::ConfigBuilder::default()
                .user("1001".to_owned())
                .exposed_ports(vec!["5432/tcp".to_owned()])
                .env(vec![
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/postgresql/15/bin".to_owned(),
                    "GOSU_VERSION=1.16".to_owned(),
                    "LANG=en_US.utf8".to_owned(),
                    "PG_MAJOR=15".to_owned(),
                    "PG_VERSION=15.4-1.pgdg120+1".to_owned(),
                    "PGDATA=/var/lib/postgresql/data".to_owned()
                ])
                .entrypoint(vec![
                    "docker-entrypoint.sh".to_owned()
                ])
                .labels(HashMap::from_iter([("maintainer".to_owned(),"someone".to_owned())]))
                .stop_signal("SIGINT".to_owned())
                .cmd(
                    vec!["postgres".to_owned()]
                )
                .volumes(vec!["/var/lib/postgresql/data".to_owned()])
                .working_dir("/postgres".to_owned())
                .build()
                .expect("Build Config")
            )
            .rootfs(image::RootFsBuilder::default()
                .typ("layers")
                .diff_ids(vec![
                    "sha256:1c3daa06574284614db07a23682ab6d1c344f09f8093ee10e5de4152a51677a1".to_owned(),
                    "sha256:310729fcb068da6941441d9627a3d8979e7dbd015c220324331e34af28b7e20c".to_owned(),
                    "sha256:6cc6868915f4c4d399ec0026fd321acfd0b92e84cd2a51076e89041b3e3118b6".to_owned()
                ]).build().expect("Rootfs"))
            .history(vec![
                image::HistoryBuilder::default()
                    .created("2023-08-15T23:39:57.178505081Z".to_owned())
                    .created_by("/bin/sh -c #(nop) ADD file:bc58956fa3d1aff2efb0264655d039fedfff28dc4ff19a65a235e82754ee1cfa in / ".to_owned())
                    .build()
                    .expect("Build History 1"),
                image::HistoryBuilder::default()
                    .created("2023-08-15T23:39:57.574431303Z".to_owned())
                    .created_by("/bin/sh -c #(nop)  CMD [\"bash\"]".to_owned())
                    .empty_layer(true)
                    .build()
                    .expect("Build History 2"),
                image::HistoryBuilder::default()
                    .created("2023-08-16T06:38:58.796057889Z".to_owned())
                    .created_by("/bin/sh -c set -eux; \tgroupadd -r postgres --gid=999; \tuseradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \tmkdir -p /var/lib/postgresql; \tchown -R postgres:postgres /var/lib/postgresql".to_owned())
                    .build()
                    .expect("Build History 3")
            ])
            .variant("v8".to_owned())
            .build()
            .expect("OCI Config Spec");

        ImageConfigurationBuilder::default()
            .docker_oci_extension(docker_oci_extension)
            .oci_spec(oci_spec)
            .build()
            .expect("Image Config")
    }

    #[test]
    fn deserialize() {
        let config_path = docker::tests::test_data_path("config.json");
        let deserialized_config =
            ImageConfiguration::from_file(config_path).expect("Could not deserialize from file");

        assert_eq!(
            deserialized_config,
            config(),
            "Deserialized config does not match expected one"
        );
    }

    #[test]
    fn serde() {
        let config_path = docker::tests::test_data_path("config.json");
        let deserialized_config = ImageConfiguration::from_file(config_path)
            .expect("Could not deserialize from file {config_path}");
        let serialized_config =
            serde_json::to_string(&deserialized_config).expect("Failed to serialize");
        let re_deserialized_config = ImageConfiguration::from_str(&serialized_config)
            .expect("Could not deserialize from serialization");

        assert_eq!(
            deserialized_config, re_deserialized_config,
            "Deserialized config from serialized config is different"
        );
    }

    #[test]
    fn test() {
        if {
            let x = true;
            x
        } {}
    }
}

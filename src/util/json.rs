//! Utility functions to help with JSON operations.

use crate::error::ParsleyResult;
use std::fs;
use std::path::Path;
use std::time::Duration;

pub(crate) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Some(Duration::from_nanos(serde::Deserialize::deserialize(
        deserializer,
    )?)))
}

pub(crate) fn serialize_duration<S>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    duration.map_or(
        Err(serde::ser::Error::custom("None value received")),
        |duration| serializer.serialize_u64(duration.as_nanos() as u64),
    )
}

pub(crate) fn merge(json1: &mut serde_json::Value, json2: serde_json::Value) {
    match (json1, json2) {
        (
            current_level_json @ &mut serde_json::Value::Object(_),
            serde_json::Value::Object(new_map_content),
        ) => {
            let merged_map = current_level_json.as_object_mut().unwrap();

            // Skip null values from the content to be added
            new_map_content
                .into_iter()
                .filter(|(_, value)| *value != serde_json::Value::Null)
                .for_each(|(key, value)| {
                    merge(
                        merged_map.entry(key).or_insert(serde_json::Value::Null),
                        value,
                    );
                });
        }
        (a, b) => *a = b,
    }
}

pub(crate) fn from_file<P, T>(path: P) -> ParsleyResult<T>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let manifest_file = std::io::BufReader::new(fs::File::open(path)?);

    Ok(serde_json::from_reader(manifest_file)?)
}

pub(crate) fn from_str<T>(s: &str) -> ParsleyResult<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_str(s)?)
}

pub(crate) fn from_slice<T>(v: &[u8]) -> ParsleyResult<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_slice(v)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case(
        "{\
            \"k1\": \"v1\",
            \"k2\": \"v2\"
        }",
        "{\
            \"k2\": \"v3\"
        }",
        "{\
            \"k1\": \"v1\",
            \"k2\": \"v3\"
        }"
        ; "Simple"
    )]
    #[test_case(
        "{\
            \"k1\": \"v1\",
            \"k2\": \"v2\"
        }",
        "{\
            \"k2\": {\
                \"k3\": \"v3\"
            }
        }",
        "{\
            \"k1\": \"v1\",
            \"k2\": {\
                \"k3\": \"v3\"
            }
        }"; "Nested"
    )]
    fn merge_cases(s1: &str, s2: &str, expected: &str) {
        let mut v1 = serde_json::Value::from_str(s1).expect("Invalid s1");
        let v2 = serde_json::Value::from_str(s2).expect("Invalid s2");
        let expected = serde_json::Value::from_str(expected).expect("Invalid expected");

        merge(&mut v1, v2);

        assert_eq!(v1, expected);
    }
}

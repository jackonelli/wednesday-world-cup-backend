//! Custom serialization of dates
use chrono::{DateTime, FixedOffset};
use serde::{self, Deserialize, Deserializer, Serializer};

/// The signature of a serialize_with function must follow the pattern:
///
///```
///fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
///where
///    S: Serializer
///```
/// although it may also be generic over the input types T.
pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    //let s = format!("{}", date.format(FORMAT));
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

/// The signature of a deserialize_with function must follow the pattern:
///
///```
///fn deserialize<'de, D>(D) -> Result<T, D::Error>
///where
///    D: Deserializer<'de>
///```
/// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)
}

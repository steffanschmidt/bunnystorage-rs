use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub mod statistics;
pub mod country;
pub mod region;
pub mod apikey;
pub mod pullzone;
pub mod storagezone;
pub mod hostname;
pub mod edgerule;
pub mod trigger;

pub fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
	return serializer.serialize_str(&dt.to_rfc3339());
}

pub fn serialize_datetime_option<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
	match dt {
		Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
		None => serializer.serialize_none()
	}
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error> where D: Deserializer<'de> {
	let datetime_str = String::deserialize(deserializer)?;
	let datetime = DateTime::parse_from_rfc3339(&datetime_str).map_err(serde::de::Error::custom)?;
	return Ok(datetime.to_utc());
}

pub fn deserialize_datetime_option<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error> where D: Deserializer<'de> {
	let datetime_str = String::deserialize(deserializer)?;
	let datetime = DateTime::parse_from_rfc3339(&datetime_str).map_err(serde::de::Error::custom)?;
	return Ok(Some(datetime.to_utc()));
}
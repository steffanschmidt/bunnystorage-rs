use chrono::{DateTime, NaiveDateTime, Utc};
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
pub mod storageendpoint;
pub mod file;

const YYYYMMDDHHMMSS: &str = "%Y-%m-%d%H:%M:%S";
const YYYYMMDDHHMMSS_MILLI: &str = "%Y-%m-%d%H:%M:%S.%f";
const YYYYMMDDHHMMSS_WEB: &str = "%Y-%m-%dT%H:%M:%S";
const YYYYMMDDHHMMSS_MILLI_WEB: &str = "%Y-%m-%dT%H:%M:%S.%f";

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
	let web_datetime_result = NaiveDateTime::parse_from_str(&datetime_str, YYYYMMDDHHMMSS_WEB);
	if let Ok(web_datetime) = web_datetime_result {
		return Ok(web_datetime.and_utc());
	}
	let web_datetime_milli_result = NaiveDateTime::parse_from_str(&datetime_str, YYYYMMDDHHMMSS_MILLI_WEB);
	if let Ok(web_datetime_milli) = web_datetime_milli_result {
		return Ok(web_datetime_milli.and_utc());
	}
	println!("{:?}", web_datetime_milli_result);
	let datetime_result = NaiveDateTime::parse_from_str(&datetime_str, YYYYMMDDHHMMSS);
	if let Ok(datetime) = datetime_result {
		return Ok(datetime.and_utc());
	}
	let datetime_milli_result = NaiveDateTime::parse_from_str(&datetime_str, YYYYMMDDHHMMSS_MILLI);
	if let Ok(datetime_milli) = datetime_milli_result {
		return Ok(datetime_milli.and_utc());
	}
	println!("{}", datetime_str);
	let rfc3339_datetime = DateTime::parse_from_rfc3339(&datetime_str)
		.map_err(serde::de::Error::custom)?;

	return Ok(rfc3339_datetime.to_utc());
}

pub fn deserialize_datetime_option<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error> where D: Deserializer<'de> {
	let datetime_str = String::deserialize(deserializer)?;
	let datetime = DateTime::parse_from_rfc3339(&datetime_str).map_err(serde::de::Error::custom)?;
	return Ok(Some(datetime.to_utc()));
}
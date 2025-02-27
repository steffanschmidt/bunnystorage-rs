use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct Region {
	pub id: i64,
	pub name: String,
	pub price_per_gigabyte: f64,
	pub region_code: String,
	pub continent_code: String,
	pub country_code: String,
	pub latitude: f64,
	pub longitude: f64,
	pub allow_latency_routing: bool,
}
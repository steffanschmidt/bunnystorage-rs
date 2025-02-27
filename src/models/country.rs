use serde::{Deserialize, Serialize};

// See https://docs.bunny.net/reference/countriespublic_getcountrylist for further documentation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct Country {
	pub name: String,
	pub iso_code: String,
	pub is_eu: bool,
	pub tax_rate: f64,
	pub tax_prefix: String,
	pub flag_url: String,
}
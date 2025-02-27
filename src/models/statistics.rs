use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// See https://docs.bunny.net/reference/statisticspublic_index for further documentation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct Statistics {
	// The total bandwidth used by the response in the given time range
	pub total_bandwidth_used: i64,
	// The total amount of traffic received from the origin
	pub total_origin_traffic: i64,
	// The median response time origin
	pub average_origin_response_time: i32,
	// The constructed origin response time chart data
	pub origin_response_time_chart: BTreeMap<String, i64>,
	// The total requests served by the response in the given time range
	pub total_requests_served: i64,
	// The average cache hit rate in the response in the given time range
	pub cache_hit_rate: f64,
	// The constructed bandwdidth used chart data
	pub bandwidth_used_chart: BTreeMap<String, i64>,
	// The constructed cached bandwidth used chart data
	pub bandwidth_cached_chart: BTreeMap<String, i64>,
	// The constructed cache hit rate chart data
	pub cache_hit_rate_chart: BTreeMap<String, i64>,
	// The constructed requests served chart data
	pub requests_served_chart: BTreeMap<String, i64>,
	// The constructed uncached requests served chart data
	pub pull_requests_pulled_chart: BTreeMap<String, i64>,
	// The constructed origin shield bandwdidth used chart data
	pub origin_shield_bandwidth_used_chart: BTreeMap<String, i64>,
	// The constructed origin shield internal bandwdidth used chart data
	pub origin_shield_internal_bandwidth_used_chart: BTreeMap<String, i64>,
	// The constructed origin traffic used chart data
	pub origin_traffic_chart: BTreeMap<String, i64>,
	// The constructed user balance history chart data
	pub user_balance_history_chart: BTreeMap<String, i64>,
	// The geo traffic distribution data
	pub geo_traffic_distribution: BTreeMap<String, i64>,
	// The constructed 3XX error responses chart data
	pub error_3xx_chart: BTreeMap<String, i64>,
	// The constructed 4XX error responses chart data
	pub error_4xx_chart: BTreeMap<String, i64>,
	// The constructed 5XX error responses chart data
	pub error_5xx_chart: BTreeMap<String, i64>,
}
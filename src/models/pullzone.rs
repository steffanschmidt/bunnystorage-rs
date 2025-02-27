use serde::{Deserialize, Serialize};

use super::{edgerule::EdgeRule, hostname::Hostname};

#[derive(Debug, Serialize, Deserialize)]
pub enum PullZoneType {
	Premium,
	Volume,
}

impl Default for PullZoneType {
	fn default() -> Self {
		return PullZoneType::Volume;
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogForwardingProtocol {
	UDP,
	TCP,
	TCPEncrypted,
	DataDog,
}

impl Default for LogForwardingProtocol {
	fn default() -> Self {
		return LogForwardingProtocol::UDP;
	}
}

// See https://docs.bunny.net/reference/pullzonepublic_index for further documentation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct PullZone {
	// The unique ID of the pull zone
	pub id: i64,
	// The name of the pull zone
	pub name: String,
	// The origin URL of the pull zone where the files are fetched from
	pub origin_url: String,
	// Determines if the Pull Zone is currently enabled, active and running
	pub enabled: bool,
	// The list of hostnames linked to this Pull Zone
	pub hostnames: Vec<Hostname>,
	// The ID of the storage zone that the pull zone is linked to
	pub storage_zone_id: i64,
	// The ID of the edge script that the pull zone is linked to
	pub edge_script_id: i64,
	// The list of referrer hostnames that are allowed to access the pull zone.Requests containing the header Referer: hostname that is not on the list will be rejected.If empty, all the referrers are allowed
	pub allowed_referrers: Vec<String>,
	// The list of referrer hostnames that are allowed to access the pull zone. Requests containing the header Referer: hostname that is not on the list will be rejected. If empty, all the referrers are allowed
	pub blocked_referrers: Vec<String>,
	// The list of IPs that are blocked from accessing the pull zone. Requests coming from the following IPs will be rejected. If empty, all the IPs will be allowed
	pub blocked_ips: Vec<String>,
	// Determines if the delivery from the North American region is enabled for this pull zone
	pub enable_geo_zone_us: bool,
	// Determines if the delivery from the European region is enabled for this pull zone
	pub enable_geo_zone_eu: bool,
	// Determines if the delivery from the Asian / Oceanian region is enabled for this pull zone
	pub enable_geo_zone_asia: bool,
	// Determines if the delivery from the South American region is enabled for this pull zone
	pub enable_geo_zone_sa: bool,
	// Determines if the delivery from the Africa region is enabled for this pull zone
	pub enable_geo_zone_af: bool,
	// True if the URL secure token authentication security is enabled
	pub zone_security_enabled: bool,
	// The security key used for secure URL token authentication
	pub zone_security_key: String,
	// True if the zone security hash should include the remote IP
	pub zone_security_include_hash_remote_ip: bool,
	// True if the Pull Zone is ignoring query strings when serving cached objects
	pub ignore_query_strings: bool,
	// The monthly limit of bandwidth in bytes that the pullzone is allowed to use
	pub monthly_bandwidth_limit: i64,
	// The amount of bandwidth in bytes that the pull zone used this month
	pub monthly_bandwidth_used: i64,
	// The total monthly charges for this so zone so far
	pub monthly_charges: i64,
	// Determines if the Pull Zone should forward the current hostname to the origin
	pub add_host_header: bool,
	// Determines the host header that will be sent to the origin
	pub origin_host_header: String,
	#[serde(rename = "type")]
	pub pull_zone_type: PullZoneType,
	// The list of extensions that will return the CORS headers
	pub access_control_origin_header_extensions: Vec<String>,
	// Determines if the CORS headers should be enabled
	pub enable_access_control_origin: bool,
	// Determines if the cookies are disabled for the pull zone
	pub disable_cookies: bool,
	// The list of budget redirected countries with the two-letter Alpha2 ISO codes
	pub budget_redirected_countries: Vec<String>,
	// The list of blocked countries with the two-letter Alpha2 ISO codes
	pub blocked_countries: Vec<String>,
	// If true the server will use the origin shield feature
	pub enable_origin_shield: bool,
	// The override cache time for the pull zone
	pub cache_control_max_age_override: i64,
	// The override cache time for the pull zone for the end client
	pub cache_control_public_max_age_override: i64,
	// Excessive requests are delayed until their number exceeds the maximum burst size
	pub burst_size: i32,
	// Max number of requests per IP per second
	pub request_limit: i32,
	// If true, access to root path will return a 403 error
	pub block_root_path_access: bool,
	// If true, POST requests to the zone will be blocked
	pub block_post_requests: bool,
	// The maximum rate at which the zone will transfer data in kb/s. 0 for unlimited
	pub limit_rate_per_second: f64,
	// The amount of data after the rate limit will be activated
	pub limit_rate_after: f64,
	// The number of connections limited per IP for this zone
	pub connection_limit_per_ip_count: i32,
	// The custom price override for this zone
	pub price_override: i64,
	// Determines if the Add Canonical Header is enabled for this Pull Zone
	pub add_canonical_header: bool,
	// Determines if the logging is enabled for this Pull Zone
	pub enable_logging: bool,
	// Determines if the cache slice (Optimize for video) feature is enabled for the Pull Zone
	pub enable_cache_slice: bool,
	// Determines if smart caching is enabled for this zone
	pub enable_smart_cache: bool,
	// The list of edge rules on this Pull Zone
	pub edge_rules: Vec<EdgeRule>,
	// Determines if the WebP Vary feature is enabled
	pub enable_web_p_vary: bool,
	// Determines if the AVIF Vary feature is enabled
	pub enable_avif_vary: bool,
	// Determines if the Country Code Vary feature is enabled
	pub enable_country_code_vary: bool,
	// Determines if the Mobile Vary feature is enabled
	pub enable_mobile_vary: bool,
	// Determines if the Cookie Vary feature is enabled
	pub enable_cookie_vary: bool,
	// Contains the list of vary parameters that will be used for vary cache by cookie string. If empty, cookie vary will not be used
	pub cookie_vary_parameters: Vec<String>,
	// Determines if the Hostname Vary feature is enabled
	pub enable_hostname_vary: bool,
	// The CNAME domain of the pull zone for setting up custom hostnames
	pub cname_domain: String,
	// Determines if the AWS Signing is enabled
	#[serde(rename = "AWSSigningEnabled")]
	pub aws_signing_enabled: bool,
	// The AWS Signing region key
	#[serde(rename = "AWSSigningKey")]
	pub aws_signing_key: String,
	// The AWS Signing region secret
	#[serde(rename = "AWSSigningSecret")]
	pub aws_signing_secret: String,
	// The AWS Signing region name
	pub aws_signing_region_name: String,
	// Determines if the TLS 1 is enabled on the Pull Zone
	pub logging_ip_anonymization_enabled: bool,
	// Determines if the TLS 1 is enabled on the Pull Zone
	pub enable_tls1: bool,
	// Determines if the TLS 1.1 is enabled on the Pull Zone
	pub enable_tls1_1: bool,
	// Determines if the Pull Zone should verify the origin SSL certificate
	pub verify_origin_ssl: bool,
	// Determines if custom error page code should be enabled
	pub error_page_enable_customer_code: bool,
	// Contains the custom error page code that will be returned
	pub error_page_custom_code: String,
	// Determines if the statuspage widget should be displayed on the error pages
	pub error_page_enable_statuspage_widget: bool,
	// The statuspage code that will be used to build the status widget
	pub error_page_statuspage_code: bool,
	// Determines if the error pages should be whitelabel or not
	pub error_page_white_label: bool,
	// The zone code of the origin shield
	pub origin_shield_zone_code: String,
	// Determines if the log forawrding is enabled
	pub log_forwating_enabled: bool,
	// The log forwarding hostname
	pub log_forwarding_host_name: String,
	// The log forwarding port
	pub log_forwarding_port: i32,
	// The log forwarding token value
	pub log_forwarding_token: String,
}
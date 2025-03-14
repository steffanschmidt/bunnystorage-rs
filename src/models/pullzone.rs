use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bunnyaiimageblueprint::BunnyAIImageBlueprint, edgerule::EdgeRule, hostname::Hostname, optimizerclass::OptimizerClass};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum PullZoneType {
	Premium,
	Volume,
}

impl Default for PullZoneType {
	fn default() -> Self {
		return PullZoneType::Premium;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
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

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum OptimizerWatermarkPosition {
	BottomLeft,
	BottomRight,
	TopLeft,
	TopRight,
	Center,
	CenterStretch,
}

impl Default for OptimizerWatermarkPosition {
	fn default() -> Self {
		return OptimizerWatermarkPosition::TopRight;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum LogAnonymizationType {
	OneDigit,
	Drop,
}

impl Default for LogAnonymizationType {
	fn default() -> LogAnonymizationType {
		return LogAnonymizationType::OneDigit;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum LogFormat {
	Plain,
	Json,
}

impl Default for LogFormat {
	fn default() -> LogFormat {
		return LogFormat::Json;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum LogForwardingFormat {
	Plain,
	Json,
}

impl Default for LogForwardingFormat {
	fn default() -> LogForwardingFormat {
		return LogForwardingFormat::Json;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ShieldDDosProtectionType {
	DetectOnly,
	ActiveStandard,
	ActiveAggressive
}

impl Default for ShieldDDosProtectionType {
	fn default() -> ShieldDDosProtectionType {
		return ShieldDDosProtectionType::ActiveStandard;
	}
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum OriginType {
	OriginUrl,
	DnsAccelerate,
	StorageZone,
	LoadBalancer,
	EdgeScript,
	MagicContainers,
	PushZone,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum PreloadingScreenTheme {
	Light,
	Dark
}

impl Default for PreloadingScreenTheme {
	fn default() -> PreloadingScreenTheme {
		return PreloadingScreenTheme::Light;
	}
}

// See https://docs.bunny.net/reference/pullzonepublic_index for further documentation
#[derive(Debug, Serialize, Deserialize, Clone)]
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
	// pub middleware_script_id: Option<String> <- Undocumented but appear in response
	// pub magic_containers_app_id: Option<String>,
	// pub magic_containers_endpoint_id: Option<String> <- Undocumented but appear in response
	// The list of referrer hostnames that are allowed to access the pull zone.Requests containing the header Referer: hostname that is not on the list will be rejected.If empty, all the referrers are allowed
	pub allowed_referrers: Vec<String>,
	// The list of referrer hostnames that are allowed to access the pull zone. Requests containing the header Referer: hostname that is not on the list will be rejected. If empty, all the referrers are allowed
	pub blocked_referrers: Vec<String>,
	// The list of IPs that are blocked from accessing the pull zone. Requests coming from the following IPs will be rejected. If empty, all the IPs will be allowed
	pub blocked_ips: Vec<String>,
	// Determines if the delivery from the North American region is enabled for this pull zone
	#[serde(alias = "EnableGeoZoneUS")]
	pub enable_geo_zone_us: bool,
	// Determines if the delivery from the European region is enabled for this pull zone
	#[serde(alias = "EnableGeoZoneEU")]
	pub enable_geo_zone_eu: bool,
	// Determines if the delivery from the Asian / Oceanian region is enabled for this pull zone
	#[serde(alias = "EnableGeoZoneASIA")]
	pub enable_geo_zone_asia: bool,
	// Determines if the delivery from the South American region is enabled for this pull zone
	#[serde(alias = "EnableGeoZoneSA")]
	pub enable_geo_zone_sa: bool,
	// Determines if the delivery from the Africa region is enabled for this pull zone
	#[serde(alias = "EnableGeoZoneAF")]
	pub enable_geo_zone_af: bool,
	// True if the URL secure token authentication security is enabled
	pub zone_security_enabled: bool,
	// The security key used for secure URL token authentication
	pub zone_security_key: String,
	// True if the zone security hash should include the remote IP
	#[serde(alias = "ZoneSecurityIncludeHashRemoteIP")]
	pub zone_security_include_hash_remote_ip: bool,
	// True if the Pull Zone is ignoring query strings when serving cached objects
	pub ignore_query_strings: bool,
	// The monthly limit of bandwidth in bytes that the pullzone is allowed to use
	pub monthly_bandwidth_limit: i64,
	// The amount of bandwidth in bytes that the pull zone used this month
	pub monthly_bandwidth_used: i64,
	// The total monthly charges for this so zone so far
	pub monthly_charges: f64,
	// Determines if the Pull Zone should forward the current hostname to the origin
	pub add_host_header: bool,
	// Determines the host header that will be sent to the origin
	pub origin_host_header: String,
	#[serde(alias = "Type")]
	pub pull_zone_type: PullZoneType,
	// The list of extensions that will return the CORS headers
	pub access_control_origin_header_extensions: Vec<String>,
	// Determines if the CORS headers should be enabled
	pub enable_access_control_origin_header: bool,
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
	#[serde(alias = "ConnectionLimitPerIPCount")]
	pub connection_limit_per_ip_count: i32,
	// The custom price override for this zone
	pub price_override: f64,
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
	#[serde(alias = "AWSSigningEnabled")]
	pub aws_signing_enabled: bool,
	// The AWS Signing region key
	#[serde(alias = "AWSSigningKey")]
	pub aws_signing_key: Option<String>,
	// The AWS Signing region secret
	#[serde(alias = "AWSSigningSecret")]
	pub aws_signing_secret: Option<String>,
	// The AWS Signing region name
	pub aws_signing_region_name: Option<String>,
	// Determines if the TLS 1 is enabled on the Pull Zone
	#[serde(alias = "LoggingIPAnonymizationEnabled")]
	pub logging_ip_anonymization_enabled: bool,
	// Determines if the TLS 1 is enabled on the Pull Zone
	#[serde(alias = "EnableTLS1")]
	pub enable_tls1: bool,
	// Determines if the TLS 1.1 is enabled on the Pull Zone
	#[serde(alias = "EnableTLS1_1")]
	pub enable_tls1_1: bool,
	// Determines if the Pull Zone should verify the origin SSL certificate
	#[serde(alias = "VerifyOriginSSL")]
	pub verify_origin_ssl: bool,
	// Determines if custom error page code should be enabled
	pub error_page_enable_custom_code: bool,
	// Contains the custom error page code that will be returned
	pub error_page_custom_code: Option<String>,
	// Determines if the statuspage widget should be displayed on the error pages
	pub error_page_enable_statuspage_widget: bool,
	// The statuspage code that will be used to build the status widget
	pub error_page_statuspage_code: Option<bool>,
	// Determines if the error pages should be whitelabel or not
	pub error_page_whitelabel: bool,
	// The zone code of the origin shield
	pub origin_shield_zone_code: String,
	// Determines if the log forawrding is enabled
	pub log_forwarding_enabled: bool,
	// The log forwarding hostname
	pub log_forwarding_host_name: Option<String>,
	// The log forwarding port
	pub log_forwarding_port: i32,
	// The log forwarding token value
	pub log_forwarding_token: Option<String>,
	pub log_forwarding_protocol: LogForwardingProtocol,
	// Determines if the permanent logging feature is enabled
	pub logging_save_to_storage: bool,
	// The ID of the logging storage zone that is configured for this Pull Zone
	pub logging_storage_zone_id: i64,
	// The ID of the video library that the zone is linked to
	pub video_library_id: i64,
	// The ID of the DNS record tied to this pull zone
	pub dns_record_id: i64,
	// The ID of the DNS zone tied to this pull zone
	pub dns_zone_id: i64,
	// The cached version of the DNS record value
	pub dns_record_value: Option<String>,
	// Determines if the optimizer should be enabled for this zone
	pub optimizer_enabled: bool,
	// Determines the maximum automatic image size for desktop clients - 0 to 5000
	pub optimizer_desktop_max_width: i32,
	// Determines the maximum automatic image size for mobile clients- 0 to 5000
	pub optimizer_mobile_max_width: i32,
	// Determines the image quality for desktop clients - 1 to 100
	pub optimizer_image_quality: i32,
	// Determines the image quality for mobile clients - 1 to 100
	pub optimizer_mobile_image_quality: i32,
	// Determines if the WebP optimization should be enabled
	pub optimizer_enable_web_p: bool,
	// Determines the image manipulation should be enabled
	pub optimizer_enable_manipulation_engine: bool,
	// Determines if the CSS minifcation should be enabled
	#[serde(alias = "OptimizerMinifyCSS")]
	pub optimizer_minify_css: bool,
	// Determines if the JavaScript minifcation should be enabled
	pub optimizer_minify_java_script: bool,
	// Determines if image watermarking should be enabled
	pub optimizer_watermark_enabled: bool,
	// Sets the URL of the watermark image
	pub optimizer_watermark_url: Option<String>,
	// Sets the offset of the watermark image
	pub optimizer_watermark_offset: f64,
	// Sets the minimum image size to which the watermark will be added
	pub optimizer_watermark_min_image_size: i32,
	// Determines if the automatic image optimization should be enabled
	pub optimizer_automatic_optimization_enabled: bool,
	// The IP of the storage zone used for Perma-Cache
	pub perma_cache_storage_zone_id: i64,
	// The number of retries to the origin server
	pub origin_retries: i32,
	// The amount of seconds to wait when connecting to the origin. Otherwise the request will fail or retry
	pub origin_connect_timeout: i32,
	// The amount of seconds to wait when waiting for the origin reply. Otherwise the request will fail or retry
	pub origin_response_timeout: i32,
	// Determines if we should use stale cache while cache is updating
	pub use_stale_while_updating: bool,
	// Determines if we should use stale cache while the origin is offline
	pub use_stale_while_offline: bool,
	// Determines if we should retry the request in case of a 5XX response
	#[serde(alias = "OriginRetry5XXResponses")]
	pub origin_retry5xx_responses: bool,
	// Determines if we should retry the request in case of a connection timeout
	pub origin_retry_connection_timeout: bool,
	// Determines if we should retry the request in case of a response timeout
	pub origin_retry_response_timeout: bool,
	// Determines the amount of time that the CDN should wait before retrying an origin request
	pub origin_retry_delay: i32,
	// Contains the list of vary parameters that will be used for vary cache by query string. If empty, all parameters will be used to construct the key
	pub query_string_vary_parameters: Vec<String>,
	// Determines if the origin shield concurrency limit is enabled
	pub origin_shield_enable_concurrency_limit: bool,
	// Determines the number of maximum concurrent requests allowed to the origin
	pub origin_shield_max_concurrent_requests: i32,
	pub enable_safe_hop: bool,
	// Determines if bunny.net should be caching error responses
	pub cache_error_responses: bool,
	// Determines the max queue wait time
	pub origin_shield_queue_max_wait_time: i32,
	// Determines the max number of origin requests that will remain in the queue
	pub origin_shield_max_queued_requests: i32,
	// Contains the list of optimizer classes
	pub optimizer_classes: Vec<OptimizerClass>,
	// Determines if the optimizer class list should be enforced
	pub optimizer_force_classes: bool,
	// Determines if cache update is performed in the background
	pub use_background_update: bool,
	// If set to true, any hostnames added to this Pull Zone will automatically enable SSL
	#[serde(alias = "EnableAutoSSL")]
	pub enable_auto_ssl: bool,
	// If set to true the query string ordering property is enabled
	pub enable_query_string_ordering: bool,
	pub log_anonymization_type: LogAnonymizationType,
	pub log_format: LogFormat,
	pub log_forwarding_format: LogForwardingFormat,
	pub shield_d_dos_protection_type: ShieldDDosProtectionType,
	pub shield_d_dos_protection_enabled: bool,
	pub origin_type: OriginType,
	// Determines if request coalescing is currently enabled
	pub enable_request_coalescing: bool,
	// Determines the lock time for coalesced requests
	pub request_coalescing_timeout: i32,
	// Returns the link short preview value for the pull zone origin connection
	pub origin_link_value: Option<String>,
	// If true, the built-in let's encrypt is disabled and requests are passed to the origin
	pub disable_lets_encrypt: bool,
	pub enable_bunny_image_ai: bool,
	pub bunny_ai_image_blueprints: Vec<BunnyAIImageBlueprint>,
	// Determines if the preloading screen is currently enabled
	pub preloading_screen_enabled: bool,
	// The custom preloading screen code
	pub preloading_screen_code: Option<String>,
	// The preloading screen logo URL
	pub preloading_screen_logo_url: Option<String>,
	// Determines if the custom preloader screen is enabled
	pub preloading_screen_code_enabled: bool,
	pub preloading_screen_theme: PreloadingScreenTheme,
	// The delay in miliseconds after which the preloading screen will be desplayed
	pub preloading_screen_delay: i32,
	// The Pull Zone specific pricing discount for EU and US region
	#[serde(alias = "EUUSDiscount")]
	pub euus_discount: i32,
	// The Pull Zone specific pricing discount for South America region
	pub south_america_discount: i32,
	// The Pull Zone specific pricing discount for Africa region
	pub africa_discount: i32,
	// The Pull Zone specific pricing discount for Asia & Oceania region
	pub asia_oceania_discount: i32,
	// The list of routing filters enabled for this zone
	pub routing_filters: Vec<String>,
	// Note: The fields below are not documented, but appear in the response
	pub block_none_referrer: bool,
	// pub sticky_session_type: u8,
	// pub sticky_session_cookie_name: Option<String>,
	// pub sticky_session_client_headers: Option<HashMap<String, String>>
	// pub user_id: String,
	// pub cache_version: i64,
	// pub optimizer_enable_upscaling: bool,
}
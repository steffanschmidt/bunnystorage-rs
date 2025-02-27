use serde::{Deserialize, Serialize};

use super::trigger::{Trigger, TriggerMatchingType};


#[derive(Debug, Serialize, Deserialize)]
pub enum EdgeRuleActionType {
	ForceSSL,
	Redirect,
	OriginUrl,
	OverrideCacheTime,
	BlockRequest,
	SetResponseHeader,
	SetRequestHeader,
	ForceDownload,
	DisableTokenAuthentication,
	EnableTokenAuthentication,
	OverrideCacheTimePublic,
	IgnoreQueryString,
	DisableOptimizer,
	ForceCompression,
	SetStatusCode,
	BypassPermaCache,
	OverrideBrowserCacheTime,
	OriginStorage,
	SetNetworkRateLimit,
	SetConnectionLimit,
	SetRequestsPerSecondLimit,
	RunEdgeScript,
	OriginMagicContainers,
	DisableWAF,
	RetryOrigin,
}

impl Default for EdgeRuleActionType {
	fn default() -> Self {
		return EdgeRuleActionType::ForceSSL
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct EdgeRule {
	// The unique GUID of the edge rule
	pub guid: String,
	pub action_type: EdgeRuleActionType,
	// The Action parameter 1. The value depends on other parameters of the edge rule.
	pub action_parameter_1: String,
	// The Action parameter 2. The value depends on other parameters of the edge rule.
	pub action_parameter_2: String,
	pub triggers: Vec<Trigger>,
	pub trigger_matching_type: TriggerMatchingType,
	// The description of the edge rule
	pub description: String,
	// Determines if the edge rule is currently enabled or not
	pub enabled: bool,
}
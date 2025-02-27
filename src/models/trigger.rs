use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerType {
	Url,
	RequestHeader,
	ResponseHeader,
	UrlExtension,
	CountryCode,
	RemoteIP,
	UrlQueryString,
	RandomChance,
	StatusCode,
	RequestMethod,
	CookieValue,
	CountryStateCode,
	OriginRetryAttemptCount,
	OriginConnectionError,
}

impl Default for TriggerType {
	fn default() -> Self {
		return TriggerType::Url;
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerPatternMatchingType {
	MatchAny,
	MatchAll,
	MatchNone,
}

impl Default for TriggerPatternMatchingType {
	fn default() -> Self {
		return TriggerPatternMatchingType::MatchNone;
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerMatchingType {
	MatchAny,
	MatchAll,
	MatchNone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trigger {
	#[serde(rename = "type")]
	pub trigger_type: TriggerType,
	// The list of pattern matches that will trigger the edge rule
	pub pattern_matches: Vec<String>,
	// The trigger parameter 1. The value depends on the type of trigger.
	pub pattern_matching_type: TriggerPatternMatchingType,
}
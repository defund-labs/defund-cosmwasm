use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    // base64 string of key
    key: String,
    offset: u64,
    limit: u64,
    count_total: bool,
    reverse: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Fund {
    key: String,
    offset: u64,
    limit: u64,
    count_total: bool,
    reverse: bool,
}
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg, CustomMsg};

/// DefundMsg is an override of CosmosMsg::Custom to add support for Defund's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefundMsg {
    EditFund {
        symbol: String,
        holdings: Vec<Holding>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Holding {
    pub token: String,
    pub percent: i64,
    pub pool_id: u64,
    pub broker_id: String,
    pub asset_type: String,
}

impl From<DefundMsg> for CosmosMsg<DefundMsg> {
    fn from(msg: DefundMsg) -> CosmosMsg<DefundMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for DefundMsg {}
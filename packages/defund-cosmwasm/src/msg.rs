use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg};

/// DefundMsg is an override of CosmosMsg::Custom to add support for Defund's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefundMsg {
    EditFund(EtfMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Holding {
    token: String,
    percent: i64,
    pool_id: u64,
    broker_id: String,
    fund_type: String,
}

/// EtfMsg captures all possible messages we can use for defund's etf module
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum EtfMsg {
    EditFund {
        symbol: String,
        holdings: Vec<Holding>
    },
}

// this is a helper to be able to return these as CosmosMsg easier
impl Into<CosmosMsg<DefundMsg>> for DefundMsg {
    fn into(self) -> CosmosMsg<DefundMsg> {
        CosmosMsg::Custom(self)
    }
}

// and another helper, so we can return DefundMsg::EditFund{..}.into() as a CosmosMsg
impl Into<CosmosMsg<DefundMsg>> for EtfMsg {
    fn into(self) -> CosmosMsg<DefundMsg> {
        CosmosMsg::Custom(DefundMsg::EditFund(self))
    }
}
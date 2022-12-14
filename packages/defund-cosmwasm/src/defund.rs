use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::Holding;

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[repr(i32)]
pub enum FundType {
    Passive = 0,
    Active = 1
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    // base64 string of key
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BaseDenom {
    pub on_defund: String,
    pub on_broker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Balances {
    pub address: Option<String>,
    pub balances: Option<Vec<Coin>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FundBalances {
    pub osmosis: Option<Balances>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Fund {
    pub symbol: String,
    pub address: String,
    pub name: String,
    pub description: String,
    pub shares: Coin,
    pub holdings: Vec<Holding>,
    pub rebalance: i64,
    pub base_denom: BaseDenom,
    pub starting_price: Coin,
    pub creator: String,
    pub rebalancing: Option<bool>,
    pub last_rebalance_height: i64,
    pub balances: FundBalances,
    pub fund_type: FundType,
    pub contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Source {
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Broker {
    id: String,
    connection_id: String,
    pools: Vec<Source>,
    status: String,
}
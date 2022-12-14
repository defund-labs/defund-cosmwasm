use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
    pub fund: String,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);
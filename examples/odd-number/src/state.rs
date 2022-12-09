use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr};
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
    pub fund: String,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);
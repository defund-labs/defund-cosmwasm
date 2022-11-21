use cosmwasm_std::{Coin, Decimal, Querier, StdResult, Uint128};

use crate::query::{};

/// This is a helper wrapper to easily use our custom queries
pub struct DefundQuerier<'a, Q: Querier> {
    querier: &'a Q,
}

impl<'a, Q: Querier> DefundQuerier<'a, Q> {
    pub fn new(querier: &'a Q) -> Self {
        DefundQuerier { querier }
    }
}
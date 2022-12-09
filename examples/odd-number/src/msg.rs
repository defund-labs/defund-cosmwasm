use crate::state::State;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    // the fund that this contract refers to
    pub fund: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// The Defund entrypoint for running your contract automatically
    Runner {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StateResponse)]
    State {},
}

// We define a custom struct for each query response
pub type StateResponse = State;
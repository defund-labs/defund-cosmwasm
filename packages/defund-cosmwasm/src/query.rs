use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, CustomQuery};

use crate::defund::{Fund, Broker, PageRequest};

/// DefundRoute is the module path route for the query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefundRoute {
    Etf,
    Broker,
}

// implement custom query
impl CustomQuery for DefundQueryWrapper {}

/// DefundQuery is an override of QueryRequest::Custom to access Defund-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefundQuery {
    //////////// Etf Module /////////////
    // Get a fund by its symbol
    GetFund { symbol: String },
    // Get all funds in store
    GetFunds { pagination: PageRequest },
    // Get current fund price in base denom by symbol
    GetFundPrice { symbol: String },

    //////////// Broker Module /////////////
    // Get a broker by its id
    GetBroker { broker: String },
    // Get all brokers in store
    GetBrokers { pagination: PageRequest }
}

/// SeiQueryWrapper is an override of QueryRequest::Custom to access Sei-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DefundQueryWrapper {
    pub route: DefundRoute,
    pub query_data: DefundQuery,
}

/// GetFundResponse is data format returned from GetFund query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundResponse {
    pub fund: Fund,
}

/// GetFundsResponse is data format returned from GetFunds query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundsResponse {
    pub funds: Vec<Fund>,
}

/// GetFundPriceResponse is data format returned from GetFundPrice query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundPriceResponse {
    pub receive: Coin,
}

/// GetBrokerResponse is data format returned from GetBroker query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBrokerResponse {
    pub broker: Broker,
}

/// GetBrokersResponse is data format returned from GetBrokers query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBrokersResponse {
    pub brokers: Vec<Broker>,
}

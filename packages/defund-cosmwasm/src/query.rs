use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, QueryRequest, CustomQuery};

use crate::defund::{Fund, Broker, PageRequest};

// implement custom query
impl CustomQuery for DefundQuery {}

/// DefundQuery is an override of QueryRequest::Custom to access Defund-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefundQuery {
    Etf(EtfQuery),
    Broker(BrokerQuery),
}

/// This contains all queries that can be made to the etf module
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EtfQuery {
    // Get a fund by its symbol
    GetFund { symbol: String },
    // Get all funds in store
    GetFunds { pagination: PageRequest },
    // Get current fund price in base denom by symbol
    GetFundPrice { symbol: String },
}

// This is a simpler way to making queries
impl Into<QueryRequest<DefundQuery>> for EtfQuery {
    fn into(self) -> QueryRequest<DefundQuery> {
        QueryRequest::Custom(DefundQuery::Etf(self))
    }
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

/// This contains all queries that can be made to the broker module
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BrokerQuery {
    // Get a broker by its id
    GetBroker { broker: String },
    // Get all brokers in store
    GetBrokers { pagination: PageRequest }
}

// This is a simpler way to making queries
impl Into<QueryRequest<DefundQuery>> for BrokerQuery {
    fn into(self) -> QueryRequest<DefundQuery> {
        QueryRequest::Custom(DefundQuery::Broker(self))
    }
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

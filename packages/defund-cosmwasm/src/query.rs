use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal, QueryRequest, CustomQuery};

use crate::defund::{Fund, PageRequest};

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

/// GetFundResponse is data format returned from SwapRequest::Simulate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundResponse {
    pub fund: Fund,
}

/// GetFundsResponse is data format returned from SwapRequest::Simulate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundsResponse {
    pub receive: Coin,
}

/// GetFundPriceResponse is data format returned from SwapRequest::Simulate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetFundPriceResponse {
    pub receive: Coin,
}

/// This contains all queries that can be made to the oracle module
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BrokerQuery {
    // ExchangeRate will return the rate between just this pair.
    ExchangeRate { offer: String, ask: String },
    // ExchangeRates will return the exchange rate between offer denom and all supported asks
    ExchangeRates { offer: String },
    // Return the tobin tax charged on exchanges with this token
    // (TODO: define if this applies to the offer or the ask?)
    TobinTax { denom: String },
}

// This is a simpler way to making queries
impl Into<QueryRequest<DefundQuery>> for BrokerQuery {
    fn into(self) -> QueryRequest<DefundQuery> {
        QueryRequest::Custom(DefundQuery::Broker(self))
    }
}

/// ExchangeRateResponse is data format returned from OracleRequest::ExchangeRate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateResponse {
    pub ask: String,
    pub rate: Decimal,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub rates: Vec<ExchangeRateResponse>,
}

/// TobinTaxResponse is data format returned from OracleRequest::TobinTax query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TobinTaxResponse {
    pub tax: Decimal,
}

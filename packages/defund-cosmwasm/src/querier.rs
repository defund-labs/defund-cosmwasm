use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::{query::{GetFundResponse, DefundQuery, GetFundsResponse, GetBrokerResponse, GetBrokersResponse, DefundQueryWrapper, DefundRoute}, defund::{PageRequest}, GetFundPriceResponse};

/// This is a helper wrapper to easily use our custom queries
pub struct DefundQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DefundQueryWrapper>,
}

impl<'a> DefundQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<DefundQueryWrapper>) -> Self {
        DefundQuerier { querier }
    }

    /*
    query etf module
    */
    pub fn query_fund(&self, symbol: String) -> StdResult<GetFundResponse> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Etf,             
            query_data: DefundQuery::GetFund { symbol } 
        };
        self.querier.query(&request.into())
    }

    pub fn query_funds(&self, key: String) -> StdResult<GetFundsResponse> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Etf,             
            query_data: DefundQuery::GetFunds { 
                pagination: PageRequest {
                    key,
                }
            }
        };

        self.querier.query(&request.into())
    }

    pub fn query_fund_price(&self, symbol: String) -> StdResult<GetFundPriceResponse> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Etf,             
            query_data: DefundQuery::GetFundPrice { 
                symbol 
            }
        };

        self.querier.query(&request.into())
    }
    /*
    query broker module
    */
    pub fn query_broker(&self, broker: String) -> StdResult<GetBrokerResponse> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Broker,             
            query_data: DefundQuery::GetBroker { 
                broker 
            }
        };
        self.querier.query(&request.into())
    }
    pub fn query_brokers(&self, key: String) -> StdResult<GetBrokersResponse> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Broker,             
            query_data: DefundQuery::GetBrokers { 
                pagination: PageRequest {
                    key,
                } 
            }
        };
        self.querier.query(&request.into())
    }
}
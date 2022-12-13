use cosmwasm_std::{QuerierWrapper, StdResult, Coin};

use crate::{query::{GetFundResponse, DefundQuery, GetFundsResponse, GetBrokerResponse, GetBrokersResponse, DefundQueryWrapper, DefundRoute}, defund::{Broker, Fund, PageRequest}};

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
    pub fn query_fund(&self, symbol: String) -> StdResult<Fund> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Etf,             
            query_data: DefundQuery::GetFund { symbol } 
        };
        let res: GetFundResponse = self.querier.query(&request.into())?;
        Ok(res.fund)
    }

    pub fn query_funds(&self, key: String) -> StdResult<Vec<Fund>> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Etf,             
            query_data: DefundQuery::GetFunds { 
                pagination: PageRequest {
                    key,
                }
            }
        };

        let res: GetFundsResponse = self.querier.query(&request.into())?;
        Ok(res.funds)
    }

    pub fn query_fund_price(&self, symbol: String) -> StdResult<Coin> {
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
    pub fn query_broker(&self, broker: String) -> StdResult<Broker> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Broker,             
            query_data: DefundQuery::GetBroker { 
                broker 
            }
        };
        let res: GetBrokerResponse = self.querier.query(&request.into())?;
        Ok(res.broker)
    }
    pub fn query_brokers(&self, key: String) -> StdResult<Vec<Broker>> {
        let request = DefundQueryWrapper {
            route: DefundRoute::Broker,             
            query_data: DefundQuery::GetBrokers { 
                pagination: PageRequest {
                    key,
                } 
            }
        };
        let res: GetBrokersResponse = self.querier.query(&request.into())?;
        Ok(res.brokers)
    }
}
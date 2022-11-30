use cosmwasm_std::{QuerierWrapper, StdResult, Coin};

use crate::{query::{GetFundResponse, DefundQuery, EtfQuery, GetFundsResponse, BrokerQuery, GetBrokerResponse, GetBrokersResponse}, defund::{Broker, Fund, PageRequest}};

/// This is a helper wrapper to easily use our custom queries
pub struct DefundQuerier<'a> {
    querier: &'a QuerierWrapper<'a, DefundQuery>,
}

impl<'a> DefundQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<DefundQuery>) -> Self {
        DefundQuerier { querier }
    }

    /*
    query etf module
    */
    pub fn query_fund<T: Into<String>>(&self, symbol: String) -> StdResult<Fund> {
        let request = EtfQuery::GetFund{ 
            symbol 
        };
        let res: GetFundResponse = self.querier.query(&request.into())?;
        Ok(res.fund)
    }

    pub fn query_funds(&self, key: String) -> StdResult<Vec<Fund>> {
        let request = EtfQuery::GetFunds { pagination: PageRequest {
            key,
        } };

        let res: GetFundsResponse = self.querier.query(&request.into())?;
        Ok(res.funds)
    }

    pub fn query_fund_price(&self, symbol: String) -> StdResult<Coin> {
        let request = EtfQuery::GetFundPrice { 
            symbol 
        };

        self.querier.query(&request.into())
    }
    /*
    query broker module
    */
    pub fn query_broker<T: Into<String>>(&self, broker: String) -> StdResult<Broker> {
        let request = BrokerQuery::GetBroker { 
            broker
        };
        let res: GetBrokerResponse = self.querier.query(&request.into())?;
        Ok(res.broker)
    }
    pub fn query_brokers<T: Into<String>>(&self, key: String) -> StdResult<Vec<Broker>> {
        let request = BrokerQuery::GetBrokers { pagination: PageRequest {
            key,
        } };
        let res: GetBrokersResponse = self.querier.query(&request.into())?;
        Ok(res.brokers)
    }
}
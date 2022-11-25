use cosmwasm_std::{Coin, Decimal, Querier, StdResult, Uint128};

use crate::{query::{GetFundResponse, DefundQuery, EtfQuery}, Fund};

/// This is a helper wrapper to easily use our custom queries
pub struct DefundQuerier<'a, Q: Querier> {
    querier: &'a Q,
}

impl<'a, Q: Querier> DefundQuerier<'a, Q> {
    pub fn new(querier: &'a QuerierWrapper<SeiQueryWrapper>) -> Self {
        DefundQuerier { querier }
    }

    /*
    query etf module
    */
    pub fn query_fund<T: Into<String>>(&self, symbol: String) -> StdResult<Fund> {
        let request = EtfQuery::GetFund{ symbol }
        .into();
        let res: GetFundResponse = self.querier.raw_query(&request.into())?;
        Ok(res.fund)
    }

    pub fn query_funds(&self, lookback_seconds: i64) -> StdResult<OracleTwapsResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::OracleTwaps { lookback_seconds },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_fund_price(&self, lookback_seconds: i64) -> StdResult<OracleTwapsResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::OracleTwaps { lookback_seconds },
        }
        .into();

        self.querier.query(&request)
    }
}
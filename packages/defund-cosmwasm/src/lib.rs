mod msg;
mod querier;
mod query;
mod defund;

pub use msg::{DefundMsg, EtfMsg, Holding};
pub use querier::DefundQuerier;
pub use query::{
    DefundQuery, GetFundResponse, GetFundsResponse, GetFundPriceResponse,
    GetBrokerResponse, GetBrokersResponse
};
pub use defund::{
    Fund, PageRequest, Source, Broker
};

// This export is added to all contracts that import this package, signifying that they require
// "defund" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_defund() {}
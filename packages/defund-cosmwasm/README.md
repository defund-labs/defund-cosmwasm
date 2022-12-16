# Defund Bindings for Cosmwasm

This is a crate for building dETFs on Defund with Cosmwasm. Add this crate to your Cargo dependencies to use the bindings to interact directly with the Defund chain.

## Install

Add the crate dependency to your `Cargo.toml` file within your smart contract
```toml
[dependencies]
defund-cosmwasm = { version = "0.1.3" }
```

## Query
Simply use the Defund querier to query funds, brokers and more.
```rust
use defund_cosmwasm::{
    DefundQuerier, GetFundResponse,
};

let querier = DefundQuerier::new(&deps.querier);

let res: GetFundResponse = querier.query_fund(state.clone().fund)?;
```
## MsgEditFund
At the end of the msg, submit an add_message to the response like so.
```rust
use defund_cosmwasm::{
    DefundMsg,
};

let res =
    Response::new()
    .add_message(DefundMsg::EditFund { 
        symbol: fund.symbol, 
        holdings: update_holdings,
    });
```
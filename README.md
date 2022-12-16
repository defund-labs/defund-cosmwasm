# Defund Cosmwasm Examples and Bindings

This is the home of all the bindings for Cosmwasm for dETF builders to integrate Defund's native chain logic into their smart contracts. This repo also contains example contracts to be used for testing and as examples for getting a feel for Defund smart contracting.

## Rebalancing
Defund will automatically run your code at each rebalance period through the `Runner` entrypoint like so. This is where the grunt work for an active Cosmwasm dETF is.
```rust
#[entry_point]
pub fn execute(
    deps: DepsMut<DefundQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DefundMsg>, ContractError> {
    match msg {
        ExecuteMsg::Runner {} => execute_runner(deps, env, info),
    }
}
```

## Store Contract
```bash
defundd tx wasm store ./tests/contracts/odd_number.wasm -y --from me --chain-id defund --gas=10000000 --fees 10000000ufetf --broadcast-mode=block
```

## Create dETF with Contract
```bash
defundd tx etf create-fund "ODD2" "The Odd 2" "If the block at the rebalance height is odd vs even swap between 75% vs 25% ATOM & OSMO." "osmo" "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2:75:osmosis:1:spot,uosmo:25:osmosis:1:spot" 25 5000000 --from me --gas=10000000 --fees 10000000ufetf -y --active --cw-id 1 --chain-id defund
```

## Query Contract
```bash
defundd query wasm contract-state smart "defund1m9l156xunmhwdr0568za49mzhdusx9uxtnevlv" '{"my_query": {}}' --from me
```
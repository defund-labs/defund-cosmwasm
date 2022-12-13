use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{StateResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, CONFIG};
use defund_cosmwasm::{
    DefundQuerier, DefundQueryWrapper, DefundMsg, Holding, GetFundResponse, Fund,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<DefundQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<DefundMsg>> {
    let state = State {
        creator: info.sender.clone(),
        owner: info.sender.clone(),
        fund: msg.fund,
    };

    CONFIG.save(deps.storage, &state)?;

    Ok(Response::default())
}

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

pub fn execute_runner(
    deps: DepsMut<DefundQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<DefundMsg>, ContractError> {
    // set updated fund details in state
    let state = CONFIG.load(deps.storage)?;
    let mut update_holdings: Vec<Holding> = vec![];

    let querier = DefundQuerier::new(&deps.querier);

    // query for fund info
    let res: GetFundResponse = querier.query_fund(state.clone().fund)?;
    let mut fund: Fund = res.clone().fund;

    // check if the current block is an odd number or even number
    if let 0=_env.block.height%2{
        // its even so set the holdings to 75% ATOM, 25% OSMO
        for mut h in fund.holdings {
            if h.token == "uosmo" {
                h.percent = 25
            }
            if h.token == "uatom" {
                h.percent = 75
            }
            update_holdings.push(h)
        }
    }
    else{
        // its odd so set the holdings to 25% ATOM, 75% OSMO
        for mut h in fund.holdings {
            if h.token == "uosmo" {
                h.percent = 75
            }
            if h.token == "uatom" {
                h.percent = 25
            }
            update_holdings.push(h)
        }
    };

    fund.holdings = update_holdings;
    CONFIG.save(deps.storage, &state)?;

    let res =
        Response::new()
        .add_message(DefundMsg::EditFund { 
            symbol: fund.symbol, 
            holdings: fund.holdings,
        });
    
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<DefundQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

fn query_state(deps: Deps<DefundQueryWrapper>) -> StdResult<StateResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_env, mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{OwnedDeps, SystemError, Coin, coins, SystemResult};
    use std::marker::PhantomData;

    // influenced by Osmosis bindings. Thank you Osmosis!
    pub fn mock_dependencies(
        contract_balance: &[Coin],
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DefundQueryWrapper>, DefundQueryWrapper> {
        let custom_querier: MockQuerier<DefundQueryWrapper> =
            MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]).with_custom_handler(|_| {
                SystemResult::Err(SystemError::InvalidRequest {
                    error: "not implemented".to_string(),
                    request: Default::default(),
                })
            });
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: custom_querier,
            custom_query_type: PhantomData,
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            fund: String::from("test")
        };
        let info = mock_info("creator", &coins(1000000, "new"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query_state(deps.as_ref()).unwrap();
        assert_eq!("test", res.fund.clone());
    }

    #[test]
    fn etf_query_works() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            fund: String::from("test")
        };
        let info = mock_info("creator", &coins(1, "phantom"));

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn etf_deserialize_fund_json() {
        let data = r#"{"symbol":"test2", "name":"Test 2", "description": "Test 2", "fund_type": 1, "address":"defund142hzmnzek9ug3f5yj4fu6y3j463xvsh3hfkj7lpr5pt4ws790fpstfwdk7","shares":{"denom":"etf/test2","amount":"0"},"holdings":[{"token":"ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2","percent":75,"pool_id":1,"broker_id":"osmosis","asset_type":"spot"},{"token":"uosmo","percent":25,"pool_id":1,"broker_id":"osmosis","asset_type":"spot"}],"rebalance":10, "rebalancing": false,"base_denom":{"on_defund":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","on_broker":"uosmo"},"starting_price":{"denom":"uosmo","amount":"10000000"},"creator":"A","last_rebalance_height":0,"contract":"defund14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s7k38kk"}"#;
        println!("{}", data);
        let p: Fund = serde_json::from_str(&data).unwrap();
        println!("{:#?}", p.symbol);
    }
}
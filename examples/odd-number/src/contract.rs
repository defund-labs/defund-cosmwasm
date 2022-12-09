use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
};

use crate::error::ContractError;
use crate::msg::{StateResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, CONFIG};
use defund_cosmwasm::{
    DefundQuerier, Fund, DefundQuery, DefundMsg, Holding,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<DefundQuery>,
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
    deps: DepsMut<DefundQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DefundMsg>, ContractError> {
    match msg {
        ExecuteMsg::Runner {} => execute_runner(deps, env, info),
    }
}

pub fn execute_runner(
    deps: DepsMut<DefundQuery>,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<DefundMsg>, ContractError> {
    // set updated fund details in state
    let state = CONFIG.load(deps.storage)?;
    let mut update_holdings: Vec<Holding> = vec![];

    // query for fund info
    let mut fund: Fund = DefundQuerier::new(&deps.querier).query_fund(state.fund.clone())?;

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
        // its even so set the holdings to 25% ATOM, 75% OSMO
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
        .add_submessage(SubMsg::new(DefundMsg::EditFund { 
            symbol: fund.symbol, 
            holdings: fund.holdings,
        }))
        .add_attributes([("action", "runner")]);
    
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<DefundQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

fn query_state(deps: Deps<DefundQuery>) -> StdResult<StateResponse> {
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
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DefundQuery>, DefundQuery> {
        let custom_querier: MockQuerier<DefundQuery> =
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
}
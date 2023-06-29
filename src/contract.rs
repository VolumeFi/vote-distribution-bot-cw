#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetJobIdResponse, InstantiateMsg, PalomaMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::CosmosMsg;
use ethabi::{Contract, Function, Param, ParamType, StateMutability, Token, Uint};
use std::collections::BTreeMap;
use std::str::FromStr;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:limit-order-bot-univ2-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        job_id: msg.job_id.clone(),
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("job_id", msg.job_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::PutVote { votes } => execute::vote(deps, votes),
    }
}

pub mod execute {
    use super::*;
    use crate::msg::Vote;
    use ethabi::Address;

    pub fn vote(
        deps: DepsMut,
        votes: Vec<Vote>,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        assert!(!votes.is_empty(), "empty votes");
        let state = STATE.load(deps.storage)?;
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "vote".to_string(),
                vec![Function {
                    name: "vote".to_string(),
                    inputs: vec![
                        Param {
                            name: "_gauge_addr".to_string(),
                            kind: ParamType::Array(Box::new(ParamType::Address)),
                            internal_type: None,
                        },
                        Param {
                            name: "_user_weight".to_string(),
                            kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                            internal_type: None,
                        },
                    ],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        let mut token_addresses: Vec<Token> = vec![];
        let mut token_weights: Vec<Token> = vec![];
        for vote in votes {
            let user_weight = vote.user_weight;
            token_addresses.push(Token::Address(
                Address::from_str(vote.gauge_address.as_str()).unwrap(),
            ));
            token_weights.push(Token::Uint(Uint::from_big_endian(
                &user_weight.to_be_bytes(),
            )))
        }
        let tokens = vec![Token::Array(token_addresses), Token::Array(token_weights)];
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_id,
                payload: Binary(
                    contract
                        .function("vote")
                        .unwrap()
                        .encode_input(tokens.as_slice())
                        .unwrap(),
                ),
            }))
            .add_attribute("action", "vote"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetJobId {} => to_binary(&query::get_job_id(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_job_id(deps: Deps) -> StdResult<GetJobIdResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetJobIdResponse {
            job_id: state.job_id,
        })
    }
}

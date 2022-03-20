use crate::error::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Addr, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, QueryRequest, StdError, StdResult, SubMsg, Uint128, WasmMsg, WasmQuery
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, Cw721Query, OwnerOfResponse};
use crate::state::{ ORDERS, Order, increment_orders };
use crate::msg::{ ExecuteMsg, InstantiateMsg };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    }
}

fn createOrder(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_address: Addr,
    token_id: String, 
    price: u128,
    expire_at: u128
) -> Result<Response, ContractError> {
    //get owner of token id
    let owner_query = Cw721QueryMsg::OwnerOf{token_id: token_id.clone(), include_expired: std::option::Option::default()};
    let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
        WasmQuery::Smart {
            contract_addr: nft_address.to_string(), 
            msg: to_binary(&owner_query)?
        })).unwrap();
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(),
        token_id: token_id.clone()
    };
    let mut messages: Vec<CosmosMsg> = vec![];
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&transfer_cw721_msg),
        funds: vec![]
    });
    let transfer_result = execute(deps.as_mut(), env, info, transfer_cw721_msg).unwrap();

    let id = increment_orders(deps.storage)?.to_string();
    let order = Order {
        token_id: token_id,
        nft_address: deps.api.addr_validate(nft_address.as_str())?,
        seller: deps.api.addr_validate(response.owner.as_str())?,
        price: price,
        expire_at: expire_at
    };
    ORDERS.save(deps.storage, &id, &order)?;
    Ok(Response::default())
}
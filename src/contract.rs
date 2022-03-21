use crate::error::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use crate::state::{ ORDERS, Order, increment_orders };
use crate::msg::{ ExecuteMsg, InstantiateMsg };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
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
        ExecuteMsg::CreateOrder{ token_id, nft_addres, price, expire_at } => create_order(deps, env, info, token_id, nft_addres, price, expire_at),

    }
}

fn create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: u128,
    expire_at: u128
) -> Result<Response, ContractError> {
    //get owner of token id
    let owner_query = Cw721QueryMsg::OwnerOf{token_id: token_id.clone(), include_expired: std::option::Option::default()};
    let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
        WasmQuery::Smart {
            contract_addr: nft_address.clone(), 
            msg: to_binary(&owner_query)?
        })).unwrap();
    
    if response.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(),
        token_id: token_id.clone(),
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![]
    };

    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    let cosmos_msgs = vec![cw721_transfer_cosmos_msg];

    let id = increment_orders(deps.storage)?.to_string();
    let order = Order {
        token_id: token_id,
        nft_address: deps.api.addr_validate(&nft_address)?,
        seller: deps.api.addr_validate(info.sender.as_str())?,
        price: price,
        expire_at: expire_at
    };
    ORDERS.save(deps.storage, &id, &order)?;
    Ok(Response::new().add_messages(cosmos_msgs).add_attributes(vec![
        ("action", "create_order"),
        ("seller", order.seller.as_str()),
        ("price", &price.to_string()),
        ("token_id", &order.token_id)
    ]))
}


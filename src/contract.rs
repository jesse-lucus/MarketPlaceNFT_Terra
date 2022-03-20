use crate::error::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use crate::state::{ ORDERS, Order, increment_orders };
use crate::msg::{ ExecuteMsg, InstantiateMsg, OrderMsg };

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
        ExecuteMsg::CreateOrder(msg) => create_order(deps, env, info, msg),

    }
}

fn create_order(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: OrderMsg
) -> Result<Response, ContractError> {
    //get owner of token id
    let owner_query = Cw721QueryMsg::OwnerOf{token_id: msg.token_id.clone(), include_expired: std::option::Option::default()};
    let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
        WasmQuery::Smart {
            contract_addr: msg.nft_address.to_string(), 
            msg: to_binary(&owner_query)?
        })).unwrap();
    
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(),
        token_id: msg.token_id.clone()
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: response.owner.to_string(),
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![],
    };

    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    let cosmos_msgs = vec![cw721_transfer_cosmos_msg];


    let id = increment_orders(deps.storage)?.to_string();
    let order = Order {
        token_id: msg.token_id,
        nft_address: deps.api.addr_validate(msg.nft_address.as_str())?,
        seller: deps.api.addr_validate(response.owner.as_str())?,
        price: msg.price,
        expire_at: msg.expire_at
    };
    ORDERS.save(deps.storage, &id, &order)?;
    Ok(Response::new().add_messages(cosmos_msgs).add_attributes(vec![
        ("action", "create_order"),
        ("seller", order.seller.as_str()),
        ("price", &msg.price.to_string()),
        ("token_id", &order.token_id.to_string())
    ]))
}
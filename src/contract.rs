use crate::error::ContractError;

use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult, Deps, Binary, Uint128
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use cw20::{Cw20ExecuteMsg};
use cw0::Expiration;

use crate::state::{ ORDERS, Order, BIDS, Bid };
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg };

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateOrder{ asset_id, nft_address, price, expire_at } => create_order(deps, env, info, asset_id, nft_address, price, expire_at),
        ExecuteMsg::CreateBid{ asset_id, nft_address, price, expire_at } => create_bid(deps, env, info, asset_id, nft_address, price, expire_at),
        ExecuteMsg::CancelOrder{ asset_id, nft_address } => cancel_order(deps, env, info, asset_id, nft_address),
        ExecuteMsg::ExecuteOrder{ asset_id, nft_address, buyer } => execute_order(deps, env, info, asset_id, nft_address, buyer)
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ValidOrder { asset_id, nft_address } => {
            let order = ORDERS.load(deps.storage, (&asset_id, &nft_address))?;
            to_binary(&order)
        }
    }
}

pub fn create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    price: Uint128,
    expire_at: Expiration
) -> Result<Response, ContractError> {

    let res = _create_order(deps, env, info, asset_id, nft_address, price, expire_at).unwrap();
    Ok(res)
    // Ok(Response::new()
    //     .add_attribute("action", "create_order")
    //     .add_attribute("seller", order.seller)
    //     .add_attribute("nft_address", order.nft_address)
    //     .add_attribute("asset_id", order.asset_id)
    //     .add_attribute("price", order.price.to_string())
    //     .add_attribute("expire_at", order.expire_at.to_string())
    // )
}

pub fn cancel_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let res = _cancel_order(deps, env, info, asset_id, nft_address).unwrap();
    Ok(res)
}

pub fn execute_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    buyer: String
) -> Result<Response, ContractError> {

    let res = _execute_order(deps, env, info, asset_id, nft_address, buyer).unwrap();
    Ok(res)
}


pub fn create_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    price: Uint128,
    expire_at: Expiration
) -> Result<Response, ContractError> {

    let res = _create_bid(deps, env, info, asset_id, nft_address, price, expire_at).unwrap();
    Ok(res)
}


fn _create_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    price: Uint128,
    expire_at: Expiration
) -> Result<Response, ContractError> {

    // TODO:  validation

    // let owner_query = Cw721QueryMsg::OwnerOf{token_id: asset_id.clone(), include_expired: std::option::Option::default()};
    // let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
    //     WasmQuery::Smart {
    //         contract_addr: nft_address.clone(), 
    //         msg: to_binary(&owner_query)?
    //     })).unwrap();
    
    let order = Order {
        asset_id: asset_id.clone(),
        nft_address: deps.api.addr_validate(&nft_address)?,
        seller: deps.api.addr_validate(info.sender.as_str())?,
        price: price,
        expire_at: expire_at
    };
    ORDERS.save(deps.storage, (&asset_id, &nft_address), &order)?;
    Ok(Response::new()
        .add_attribute("action", "create_order")
        .add_attribute("asset_id", order.asset_id)
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("seller", order.seller)
        .add_attribute("price", order.price)
    )
}

fn _create_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    price: Uint128,
    expire_at: Expiration
) -> Result<Response, ContractError> {
    let order = ORDERS.load(deps.storage, (&asset_id, &nft_address))?;
    if order.expire_at.is_expired(&env.block) {
        return Err(ContractError::Expired {})
    }
    if order.price > price {
        return Err(ContractError::MinPrice { min_bid_amount: price })
    }

    if !BIDS.has(deps.storage, (&asset_id, &nft_address)) {
        let bid = Bid {
            asset_id: asset_id.clone(),
            nft_address: deps.api.addr_validate(&nft_address)?,
            bidder: deps.api.addr_validate(info.sender.as_str())?,
            seller: order.seller,
            price: price,
            expire_at: expire_at
        };
        BIDS.save(deps.storage, (&asset_id, &nft_address), &bid)?;    
    } else {
        let mut bid = BIDS.load(deps.storage, (&asset_id, &nft_address))?;
        if bid.price > price {
            return Err(ContractError::MinPrice { min_bid_amount: price })
        }
        bid.bidder = deps.api.addr_validate(info.sender.as_str())?;
        bid.price = price;
        BIDS.save(deps.storage, (&asset_id, &nft_address), &bid)?;
    }

    Ok(Response::new()
        // .add_messages(cosmos_msgs)
        .add_attribute("action", "create_bid")
        .add_attribute("asset_id", asset_id)
        .add_attribute("nft_address", nft_address)
        .add_attribute("bidder", info.sender.to_string())
        .add_attribute("price", price)
    )
}

fn _cancel_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    // if !ORDERS.has(deps.storage, (&asset_id, &nft_address)) {
    //     return Err(ContractError::Unauthorized {});
    // }
    let order = ORDERS.load(deps.storage, (&asset_id, &nft_address))?;

    // only seller cancel order
    if order.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    //todo refund bid

    //return nft to seller
    let cosmos_msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: order.nft_address.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
          recipient: order.seller.to_string(), 
          token_id: order.asset_id
        })?,
        funds: vec![]
      })];

    //remove order
    ORDERS.remove(deps.storage, (&asset_id, &nft_address));
    Ok(Response::new()
        .add_messages(cosmos_msgs)
        .add_attribute("action", "cancel_order")
        .add_attribute("asset_id", asset_id)
        .add_attribute("nft_address", nft_address)
    )
}

fn _execute_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    buyer: String
) -> Result<Response, ContractError> {

    if !ORDERS.has(deps.storage, (&asset_id, &nft_address)) {
        return Err(ContractError::Unauthorized {});
    }
    let order = ORDERS.load(deps.storage, (&asset_id, &nft_address))?;

    // create transfer cw20 msg
    let transfer_cw20_msg = Cw20ExecuteMsg::Transfer {
        recipient: order.seller.to_string(),
        amount: order.price,
    };
    let exec_cw20_transfer = WasmMsg::Execute {
        contract_addr: info.sender.clone().to_string(), // TODOD convert token Address
        msg: to_binary(&transfer_cw20_msg)?,
        funds: vec![],
    };
    
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: buyer,
        token_id: asset_id.clone(),
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: nft_address,
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![]
    };

    let cw20_transfer_cosmos_msg: CosmosMsg = exec_cw20_transfer.into();

    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    let cosmos_msgs = vec![cw20_transfer_cosmos_msg, cw721_transfer_cosmos_msg];
    
    Ok(Response::new()
        .add_messages(cosmos_msgs)
        .add_attribute("action", "execute_order")
        .add_attribute("asset_id", order.asset_id)
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("seller", order.seller)
        .add_attribute("price", order.price)
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    mod instantiate {
        use super::*;
        use crate::error::ContractError;

        #[test]
        fn works() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                decimals: Uint128::from(11223344u128),
                name: "testing2".to_string(),
                symbol: "testing".to_string()
            };
            let res = instantiate(deps.as_mut(), mock_env(), mock_info(&"signer".to_string(), &[]), instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());

        }
    }

    #[test]
    fn _create_order_works() {
        let mut deps = mock_dependencies(&[]);
        let res = _create_order(
            deps.as_mut(),
            mock_env(),
            mock_info(&"signer".to_string(), &[]),
            "47850".to_string(),
            "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
            Uint128::from(11223344u128),
            Uint128::from(11223344u128)
        ).unwrap();
        assert_eq!(res.asset_id, "47850".to_string());

        // let cancel_res = _cancel_order(
        //     deps.as_mut(),
        //     mock_env(),
        //     mock_info(&"signer".to_string(), &[]),
        //     "47850".to_string(),
        //     "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
        // ).unwrap();
        // assert_eq!(cancel_res.asset_id, "47850".to_string());
    }
}
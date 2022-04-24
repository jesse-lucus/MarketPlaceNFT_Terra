use crate::error::ContractError;

use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult, Deps, Binary, Uint128, Timestamp
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use cw20::{Cw20ExecuteMsg};
use cw0:: Expiration;

use crate::state::{ ORDERS, Order, BIDS, Bid, Config, CONFIG };
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg };
use crate::asset::{Asset, AssetInfo};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let con = Config {
        accepted_token: deps.api.addr_validate(&msg.accepted_token)?
    };
    CONFIG.save(deps.storage, &con)?;
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
        ExecuteMsg::CreateOrder{ token_id, nft_address, price, expire_at } => create_order(deps, env, info, token_id, nft_address, price, expire_at),
        ExecuteMsg::CreateBid{ token_id, nft_address, price, expire_at } => create_bid(deps, env, info, token_id, nft_address, price, expire_at),
        ExecuteMsg::CancelOrder{ token_id, nft_address } => cancel_order(deps, env, info, token_id, nft_address),
        ExecuteMsg::CancelBid{ token_id, nft_address } => cancel_bid(deps, env, info, token_id, nft_address),
        ExecuteMsg::ExecuteOrder{ token_id, nft_address } => execute_order(deps, env, info, token_id, nft_address)
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Version {} => {
            // let seconds = env.block.time;
            to_binary(&"1.72".to_string())
        }

        QueryMsg::ValidOrder { token_id, nft_address } => {
            let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;
            to_binary(&order)
        }
        QueryMsg::ValidBid { token_id, nft_address } => {
            let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;
            to_binary(&bid)
        }
    }
}

pub fn create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
) -> Result<Response, ContractError> {

    let res = _create_order(deps, env, info, token_id, nft_address, price, expire_at).unwrap();
    Ok(res)
    // Ok(Response::new()
    //     .add_attribute("action", "create_order")
    //     .add_attribute("seller", order.seller)
    //     .add_attribute("nft_address", order.nft_address)
    //     .add_attribute("token_id", order.token_id)
    //     .add_attribute("price", order.price.to_string())
    //     .add_attribute("expire_at", order.expire_at.to_string())
    // )
}

pub fn cancel_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let res = _cancel_order(deps, env, info, token_id, nft_address).unwrap();
    Ok(res)
}

pub fn execute_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let res = _execute_order(deps, env, info, token_id, nft_address).unwrap();
    Ok(res)
}


pub fn create_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
) -> Result<Response, ContractError> {

    let res = _create_bid(deps, env, info, token_id, nft_address, price, expire_at).unwrap();
    Ok(res)
}

pub fn cancel_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let res = _cancel_bid(deps, env, info, token_id, nft_address).unwrap();
    Ok(res)
}

fn _create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
) -> Result<Response, ContractError> {
    let owner_res: OwnerOfResponse =
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_address.clone(),
        msg: to_binary(&Cw721QueryMsg::OwnerOf { token_id: token_id.clone(), include_expired: std::option::Option::default() })?,
    })).unwrap();
    if owner_res.owner != info.sender.to_string() {
        return Err(ContractError::NoOwner {})
    }
    if price.amount <= Uint128::zero() {
        return Err(ContractError::InvalidPrice {})
    }
    match expire_at {
        Expiration::AtHeight(_) => {},
        Expiration::AtTime(time) => {
            let seconds = env.block.time.seconds();
            if time.seconds() < seconds + 60u64 {
                return Err(ContractError::InvalidExpiration {})
            }
        },
        Expiration::Never {} => {},
    }
    //get NFT asset to seller - should be called from frontend
    let order = Order {
        token_id: token_id.clone(),
        nft_address: deps.api.addr_validate(&nft_address)?,
        seller: deps.api.addr_validate(info.sender.as_str())?,
        price: price,
        expire_at: expire_at
    };
    ORDERS.save(deps.storage, (&token_id, &nft_address), &order)?;
    Ok(Response::new()
        .add_attribute("action", "create_order")
        .add_attribute("token_id", order.token_id)
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("seller", order.seller)
        .add_attribute("price", order.price.amount)
    )
}

fn _create_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
) -> Result<Response, ContractError> {
    let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;
    if order.expire_at.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }
    if order.price.amount > price.amount {
        return Err(ContractError::MinPrice { min_bid_amount: price.amount })
    }
    // price.assert_sent_native_token_balance(&info)?;
    let mut messages: Vec<CosmosMsg> = vec![];
    if !BIDS.has(deps.storage, (&token_id, &nft_address)) {
        let bid = Bid {
            token_id: token_id.clone(),
            nft_address: deps.api.addr_validate(&nft_address)?,
            bidder: deps.api.addr_validate(info.sender.as_str())?,
            seller: order.seller,
            price: price,
            expire_at: expire_at
        };
        BIDS.save(deps.storage, (&token_id, &nft_address), &bid)?;    
    } else {
        let mut bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;
        if bid.price.amount > price.amount {
            return Err(ContractError::MinPrice { min_bid_amount: price.amount.clone() })
        }
        //refund escrow to previous bidder
        messages.push(bid.price.clone().into_msg(&deps.querier, bid.bidder.clone())?);

        bid.bidder = deps.api.addr_validate(info.sender.as_str())?;
        bid.price = price;
        BIDS.save(deps.storage, (&token_id, &nft_address), &bid)?;
    }

    //TODO price to Escrow - should be performed from frontend

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "create_bid")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
        .add_attribute("bidder", info.sender.to_string())
    )
}

fn _cancel_bid(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    //refund escrow money to bidder and cancel bid
    let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;
    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(bid.price.into_msg(&deps.querier, bid.bidder)?);

    BIDS.remove(deps.storage, (&token_id, &nft_address));
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "cancel_bid")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
    )
}

fn _cancel_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;

    // only seller cancel order
    if order.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let mut messages: Vec<CosmosMsg> = vec![];

    if BIDS.has(deps.storage, (&token_id, &nft_address)) {
        //refund escrow money to bidder and cancel bid
        let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;
        messages.push(bid.price.into_msg(&deps.querier, bid.bidder)?);
        BIDS.remove(deps.storage, (&token_id, &nft_address));
    }
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: order.nft_address.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
          recipient: order.seller.to_string(), 
          token_id: order.token_id
        })?,
        funds: vec![]
      })
    );

    //remove order
    ORDERS.remove(deps.storage, (&token_id, &nft_address));
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "cancel_order")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
    )
}

fn _execute_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    if !ORDERS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::Unauthorized {});
    }
    let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;

    // only seller approve order
    if order.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if !BIDS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::NoBid {});
    }
    let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;

    let mut messages: Vec<CosmosMsg> = vec![];

    // send bid amount to seller
    messages.push(bid.price.into_msg(&deps.querier, order.seller.clone())?);


    // send nft to bidder
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: order.nft_address.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
          recipient: bid.bidder.to_string(), 
          token_id: order.token_id.clone()
        })?,
        funds: vec![]
      })
    );

    // remove bids and orders
    BIDS.remove(deps.storage, (&token_id, &nft_address));
    ORDERS.remove(deps.storage, (&token_id, &nft_address));

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "execute_order")
        .add_attribute("token_id", order.token_id)
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("seller", order.seller.clone())
        .add_attribute("bidder", bid.bidder.clone())
        .add_attribute("price", format!("{}", order.price))
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
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
        let expiration = Expiration::AtTime(Timestamp::from_seconds(1648938996));

        let price = Asset {
            amount: Uint128::from(1u128),
            info: AssetInfo::NativeToken {denom : "uluna".to_string()}
        };

        let res = _create_order(
            deps.as_mut(),
            mock_env(),
            mock_info(&"signer".to_string(), &[]),
            "47850".to_string(),
            "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
            price,
            expiration
        ).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn _create_bid_works() {
        let mut deps = mock_dependencies(&[]);
        let expiration = Expiration::AtTime(Timestamp::from_seconds(1648938996));

        let price = Asset {
            amount: Uint128::from(1u128),
            info: AssetInfo::NativeToken {denom : "uluna".to_string()}
        };

        let order_res = _create_order(
            deps.as_mut(),
            mock_env(),
            mock_info(&"signer".to_string(), &[]),
            "47850".to_string(),
            "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
            price.clone(),
            expiration.clone()
        ).unwrap();
        assert_eq!(0, order_res.messages.len());


        let res = _create_bid(
            deps.as_mut(),
            mock_env(),
            mock_info(&"signer".to_string(), &[]),
            "47850".to_string(),
            "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
            price,
            expiration
        ).unwrap();
        assert_eq!(0, res.messages.len());
    }

}
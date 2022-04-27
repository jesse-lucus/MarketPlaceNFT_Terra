use crate::error::ContractError;

use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult, Deps, Binary, Uint128,
    Storage, QuerierWrapper, Decimal
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use cw0:: Expiration;

use crate::state::{ ORDERS, Order, BIDS, Bid, Config, CONFIG, PAUSED };
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg };
use crate::asset::{ Asset };

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let con = Config {
        owner: info.sender,
        accepted_token: deps.api.addr_validate(&msg.accepted_token)?,
        owner_cut_rate: msg.owner_cut_rate,
        owner_cut_rate_max: Decimal::percent(10),
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
        ExecuteMsg::SetPaused { paused } => set_paused(deps, env, info, paused),
        ExecuteMsg::CreateOrder{ token_id, nft_address, price, expire_at } => create_order(deps, env, info, token_id, nft_address, price, expire_at),
        ExecuteMsg::UpdateOrder{ token_id, nft_address, price, expire_at } => update_order(deps, env, info, token_id, nft_address, price, expire_at),
        ExecuteMsg::CreateBid{ token_id, nft_address, price, expire_at } => create_bid(deps, env, info, token_id, nft_address, price, expire_at),
        ExecuteMsg::CancelOrder{ token_id, nft_address } => cancel_order(deps, env, info, token_id, nft_address),
        ExecuteMsg::CancelBid{ token_id, nft_address } => cancel_bid(deps, env, info, token_id, nft_address),
        ExecuteMsg::SafeExecuteOrder{ token_id, nft_address, price } => safe_execute_order(deps, env, info, token_id, nft_address, price),
        ExecuteMsg::AcceptBid{ token_id, nft_address, price } => accept_bid(deps, env, info, token_id, nft_address, price)
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

pub fn set_paused(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    paused: bool,
) -> Result<Response, ContractError> {
    PAUSED.save(deps.storage, &paused)?;
    Ok(Response::new()
        .add_attribute("action", "set_paused")
        .add_attribute("paused", paused.to_string())
    )
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
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let owner_res: OwnerOfResponse =
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_address.clone(),
        msg: to_binary(&Cw721QueryMsg::OwnerOf { token_id: token_id.clone(), include_expired: std::option::Option::default() })?,
    })).unwrap();
    if owner_res.owner != info.sender.to_string() {
        return Err(ContractError::NoOwner {})
    }
    let res = _create_order(deps, env, info, token_id, nft_address, price, expire_at).unwrap();
    Ok(res)
}

pub fn update_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
) -> Result<Response, ContractError> {
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let res = _update_order(deps, env, info, token_id, nft_address, price, expire_at).unwrap();
    Ok(res)
}

pub fn cancel_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let res = _cancel_order(deps, env, info, token_id, nft_address).unwrap();
    Ok(res)
}

pub fn safe_execute_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset
) -> Result<Response, ContractError> {
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let res = _safe_execute_order(deps, env, info, token_id, nft_address, price).unwrap();
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
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let res = _create_bid(deps, env, info, token_id, nft_address, price, expire_at).unwrap();
    Ok(res)
}

pub fn accept_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset
) -> Result<Response, ContractError> {
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let res = _accept_bid(deps, env, info, token_id, nft_address, price).unwrap();
    Ok(res)
}

pub fn cancel_bid(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    token_id: String,
    nft_address: String
) -> Result<Response, ContractError> {
    if PAUSED.load(deps.storage)? {
        return Err(ContractError:: MarketplacePaused{});
    }
    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(_cancel_bid(deps.storage, &deps.querier, token_id.clone(), nft_address.clone()).unwrap());
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "cancel_bid")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
    )
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
    match order.expire_at {
        Expiration::AtHeight(_) => {},
        Expiration::AtTime(time) => {
            let seconds = env.block.time.seconds();
            if time.seconds() < seconds {
                return Err(ContractError::Expired {});
            }
        },
        Expiration::Never {} => {},
    }

    if order.price.amount > price.amount {
        return Err(ContractError::MinPrice { min_bid_amount: price.amount })
    }
    let mut messages: Vec<CosmosMsg> = vec![];

    let has_bid = BIDS.has(deps.storage, (&token_id, &nft_address));
    if has_bid {
        let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;
        match bid.expire_at {
            Expiration::AtHeight(_) => {},
            Expiration::AtTime(time) => {
                let seconds = env.block.time.seconds();
                if time.seconds() < seconds {
                    if price.amount <= Uint128::zero() {
                        return Err(ContractError::ZeroBidAmount {});
                    }            
                } else {
                    if price.amount < bid.price.amount {
                        return Err(ContractError::InvalidBidAmount {});
                    }
                }
            },
            Expiration::Never {} => {},
        }
        messages.push(_cancel_bid(deps.storage, &deps.querier, token_id.clone(), nft_address.clone()).unwrap())
    } else {
        if price.amount <= Uint128::zero() {
            return Err(ContractError::ZeroBidAmount {});
        }
    }

    //Transfer sale amount from bidder escrow- should be done from coin params on execution
    let bid = Bid {
        token_id: token_id.clone(),
        nft_address: deps.api.addr_validate(&nft_address)?,
        bidder: deps.api.addr_validate(info.sender.as_str())?,
        seller: order.seller,
        price: price,
        expire_at: expire_at
    };
    BIDS.save(deps.storage, (&token_id, &nft_address), &bid)?;
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "create_bid")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
        .add_attribute("bidder", info.sender.to_string())
    )
}

/**
 * @dev Cancel an already published order
 *  can only be canceled by seller or the contract owner
 */
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
    let has_bid = BIDS.has(deps.storage, (&token_id, &nft_address));
    if has_bid {
        messages.push(_cancel_bid(deps.storage, &deps.querier, token_id.clone(), nft_address.clone()).unwrap())
    }

    //  send asset back to seller
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

fn _update_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset,
    expire_at: Expiration
    
) -> Result<Response, ContractError> {

    if !ORDERS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::NoOrder {});
    }
    let mut order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;
    match order.expire_at {
        Expiration::AtHeight(_) => {},
        Expiration::AtTime(time) => {
            let seconds = env.block.time.seconds();
            if time.seconds() < seconds {
                return Err(ContractError::Expired {});
            } 
        },
        Expiration::Never {} => {},
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
    // only seller update order
    if order.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    order.price = price.clone();
    order.expire_at = expire_at.clone();
    Ok(Response::new()
        .add_attribute("action", "update_order")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
    )
}

fn _cancel_bid(
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
    token_id: String,
    nft_address: String
) -> StdResult<CosmosMsg> {
    let bid = BIDS.load(storage, (&token_id, &nft_address))?;
    let message = bid.price.into_msg(querier, bid.bidder)?;
    BIDS.remove(storage, (&token_id, &nft_address));
    Ok(message)
}

fn _safe_execute_order(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset
) -> Result<Response, ContractError> {

    if !ORDERS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::NoOrder {});
    }
    let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;
    if order.price.info != price.info || order.price.amount != price.amount {
        return Err(ContractError::InvalidPrice {});
    }
    // Transfer all amount by coin param on calling
    // it should be performed from frontend by coin params.
    let mut messages: Vec<CosmosMsg> = vec![];
    let con = CONFIG.load(deps.storage)?;
    if con.owner_cut_rate > Decimal::zero() {
        let sales_share_amount_asset = Asset {
            info: order.price.info.clone(),
            amount: order.price.amount * con.owner_cut_rate
        };
        messages.push(sales_share_amount_asset.into_msg(&deps.querier, con.owner.clone())?);
    }
    let seller_amount_asset = Asset {
        info: order.price.info.clone(),
        amount: order.price.amount - (order.price.amount * con.owner_cut_rate)
    };
    messages.push(seller_amount_asset.into_msg(&deps.querier, order.seller.clone())?);

    // remove bids and orders
    if BIDS.has(deps.storage, (&token_id, &nft_address)) {
        messages.push(_cancel_bid(deps.storage, &deps.querier, token_id.clone(), nft_address.clone()).unwrap());
    }
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: order.nft_address.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
          recipient: info.sender.to_string(), 
          token_id: order.token_id.clone()
        })?,
        funds: vec![]
      })
    );
    ORDERS.remove(deps.storage, (&token_id, &nft_address));
    Ok(Response::new()
        .add_attribute("action", "_safe_execute_order")
        .add_attribute("token_id", token_id)
        .add_attribute("nft_address", nft_address)
    )
}

fn _accept_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    nft_address: String,
    price: Asset
) -> Result<Response, ContractError> {

    let con = CONFIG.load(deps.storage)?;

    //Frontend -  send coin amount as param
    if !ORDERS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::NoOrder {});
    }
    let order = ORDERS.load(deps.storage, (&token_id, &nft_address))?;
    // only seller approve order
    if order.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    match order.expire_at {
        Expiration::AtHeight(_) => {},
        Expiration::AtTime(time) => {
            let seconds = env.block.time.seconds();
            if time.seconds() < seconds {
                return Err(ContractError::Expired {})
            }
        },
        Expiration::Never {} => {},
    }    

    if !BIDS.has(deps.storage, (&token_id, &nft_address)) {
        return Err(ContractError::NoBid {});
    }
    let bid = BIDS.load(deps.storage, (&token_id, &nft_address))?;

    // price validation - native sent balance check
    price.assert_sent_native_token_balance(&info)?;

    if bid.price.info != price.info || bid.price.amount != price.amount {
        return Err(ContractError::InvalidPrice {});
    }

    match bid.expire_at {
        Expiration::AtHeight(_) => {},
        Expiration::AtTime(time) => {
            let seconds = env.block.time.seconds();
            if time.seconds() < seconds {
                return Err(ContractError::BidExpired {})
            }
        },
        Expiration::Never {} => {},
    }


    let mut messages: Vec<CosmosMsg> = vec![];

    // transfer escrowed bid amount minus market fee to seller
    let seller_amount_asset = Asset {
        info: bid.price.info.clone(),
        amount: bid.price.amount - (order.price.amount * con.owner_cut_rate)
    };
    messages.push(seller_amount_asset.into_msg(&deps.querier, order.seller.clone())?);

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
    use crate::msg::{ InstantiateMsg };
    use crate::asset::{ Asset, AssetInfo };
    use cosmwasm_std::{ Timestamp };

    mod instantiate {
        use super::*;

        #[test]
        fn works() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                decimals: Uint128::from(11223344u128),
                name: "testing2".to_string(),
                symbol: "testing".to_string(),
                accepted_token: "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp".to_string(),
                owner_cut_rate: Decimal::from_ratio(Uint128::from(10u64), Uint128::from(100u64))
            };
            let res = instantiate(deps.as_mut(), mock_env(), mock_info(&"signer".to_string(), &[]), instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());

        }
    }

    #[test]
    fn create_order_works() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(&"signer".to_string(), &[]);
        let env = mock_env();
        let expiration = Expiration::AtTime(Timestamp::from_seconds(1648958996));
        let expired_expiration = Expiration::AtTime(env.block.time);

        let price = Asset {
            amount: Uint128::from(10000u128),
            info: AssetInfo::NativeToken {denom : "uluna".to_string()}
        };
        let zeroprice = Asset {
            amount: Uint128::from(0u64),
            info: AssetInfo::NativeToken {denom : "uluna".to_string()}
        };

        let nft_address = "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju".to_string();
        let token_id = "2".to_string();

        let zero_price_err = _create_order(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            token_id.clone(),
            nft_address.clone(),
            zeroprice.clone(),
            expiration
        ).unwrap_err();
        assert_eq!(zero_price_err, ContractError::InvalidPrice {});

        let expired_err = _create_order(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            token_id.clone(),
            nft_address.clone(),
            price.clone(),
            expired_expiration
        ).unwrap_err();
        assert_eq!(expired_err, ContractError::InvalidExpiration {});

        let res = _create_order(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            token_id.clone(),
            nft_address.clone(),
            price.clone(),
            expiration
        ).unwrap();
        assert_eq!(res, Response::new()
            .add_attribute("action", "create_order")
            .add_attribute("token_id", token_id)
            .add_attribute("nft_address", nft_address)
            .add_attribute("seller", info.sender)
            .add_attribute("price", price.amount)
        );
    }

    // #[test]
    // fn _create_bid_works() {
    //     let mut deps = mock_dependencies(&[]);
    //     let expiration = Expiration::AtTime(Timestamp::from_seconds(1648938996));

    //     let price = Asset {
    //         amount: Uint128::from(10000u128),
    //         info: AssetInfo::NativeToken {denom : "uluna".to_string()}
    //     };

    //     let order_res = _create_order(
    //         deps.as_mut(),
    //         mock_env(),
    //         mock_info(&"signer".to_string(), &[]),
    //         "47850".to_string(),
    //         "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
    //         price.clone(),
    //         expiration.clone()
    //     ).unwrap();
    //     assert_eq!(0, order_res.messages.len());


    //     let res = _create_bid(
    //         deps.as_mut(),
    //         mock_env(),
    //         mock_info(&"signer".to_string(), &[]),
    //         "47850".to_string(),
    //         "terra13rxnrpjk5l8c77zsdzzq63zmavu03hwn532wn0".to_string(),
    //         price,
    //         expiration
    //     ).unwrap();
    //     assert_eq!(0, res.messages.len());
    // }

}
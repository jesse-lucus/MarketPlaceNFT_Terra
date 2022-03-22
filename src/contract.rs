use crate::error::ContractError;

use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, CosmosMsg, Response, QueryRequest, WasmMsg, WasmQuery, StdResult, Deps, Binary, Uint128
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};
use cw20::{Cw20ExecuteMsg};

use crate::state::{ ORDERS, Order };
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
        ExecuteMsg::CancelOrder{ asset_id, nft_address } => cancel_order(deps, env, info, asset_id, nft_address),
        ExecuteMsg::ExecuteOrder{ asset_id, nft_address, buyer } => execute_order(deps, env, info, asset_id, nft_address, buyer)
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::ValidOrder { asset_id: _, nft_address: _ } => {
            let out = to_binary("success OK!")?;
            Ok(out)
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
    expire_at: Uint128
) -> Result<Response, ContractError> {

    let order = _create_order(deps, env, info, asset_id, nft_address, price, expire_at).unwrap();
    Ok(Response::new()
        .add_attribute("action", "create_order")
        .add_attribute("seller", order.seller)
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("asset_id", order.asset_id)
        .add_attribute("price", order.price.to_string())
        .add_attribute("expire_at", order.expire_at.to_string())
    )
}

pub fn cancel_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String
) -> Result<Response, ContractError> {

    let order = _cancel_order(deps, env, info, asset_id, nft_address).unwrap();
    Ok(Response::new()
        .add_attribute("action", "cancel_order")
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("asset_id", order.asset_id)
    )
}

pub fn execute_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    buyer: String
) -> Result<Response, ContractError> {

    let order = _execute_order(deps, env, info, asset_id, nft_address, buyer).unwrap();
    Ok(Response::new()
        .add_attribute("action", "execute_order")
        .add_attribute("nft_address", order.nft_address)
        .add_attribute("asset_id", order.asset_id)
    )
}


fn _create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    price: Uint128,
    expire_at: Uint128
) -> Result<Order, ContractError> {
    // let owner_query = Cw721QueryMsg::OwnerOf{token_id: asset_id.clone(), include_expired: std::option::Option::default()};
    // let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
    //     WasmQuery::Smart {
    //         contract_addr: nft_address.clone(), 
    //         msg: to_binary(&owner_query)?
    //     })).unwrap();
    
    // if response.owner != info.sender {
    //     return Err(ContractError::Unauthorized {});
    // }
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: env.contract.address.to_string(),
        token_id: asset_id.clone(),
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: nft_address.clone(),
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![]
    };

    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    let _cosmos_msgs = vec![cw721_transfer_cosmos_msg];

    // let id = increment_orders(deps.storage)?.to_string();
    let order = Order {
        asset_id: asset_id.clone(),
        nft_address: deps.api.addr_validate(&nft_address)?,
        seller: deps.api.addr_validate(info.sender.as_str())?,
        price: price,
        expire_at: expire_at
    };
    ORDERS.save(deps.storage, ("asset_id", "nft_address"), &order)?;
    Ok(order.clone())
}

fn _cancel_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String
) -> Result<Order, ContractError> {

    // let owner_query = Cw721QueryMsg::OwnerOf{token_id: asset_id.clone(), include_expired: std::option::Option::default()};
    // let response: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(
    //     WasmQuery::Smart {
    //         contract_addr: nft_address.clone(), 
    //         msg: to_binary(&owner_query)?
    //     })).unwrap();
    
    // if response.owner != info.sender {
    //     return Err(ContractError::Unauthorized {});
    // }

    if !ORDERS.has(deps.storage, (&asset_id, &nft_address)) {
        return Err(ContractError::Unauthorized {});
    }
    let order = ORDERS.load(deps.storage, (&asset_id, &nft_address))?;
    
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: order.seller.to_string(),
        token_id: asset_id.clone(),
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: nft_address.to_string(),
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![]
    };

    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    let _cosmos_msgs = vec![cw721_transfer_cosmos_msg];
    ORDERS.remove(deps.storage, (&asset_id, &nft_address));
    Ok(order)
}

fn _execute_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    asset_id: String,
    nft_address: String,
    buyer: String
) -> Result<Order, ContractError> {

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
        contract_addr: info.sender.clone().to_string(),
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

    let _cosmos_msgs = vec![cw20_transfer_cosmos_msg, cw721_transfer_cosmos_msg];
    Ok(order)
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
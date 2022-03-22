use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ StdResult, Storage, Addr };
use cw_storage_plus::{ Item, Map };

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Order {
    pub asset_id: String,
    pub nft_address: Addr,
    pub seller: Addr,
    pub price: u128,
    pub expire_at: u128
}

pub const ORDERS: Map<(&str, &str), Order> = Map::new("orders");
pub const ORDERS_COUNT: Item<u64> = Item::new("num_orders");

pub fn num_orders(storage: &mut dyn Storage) -> StdResult<u64> {
    Ok(ORDERS_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_orders(storage: &mut dyn Storage) -> StdResult<u64> {
    let val = num_orders(storage)? + 1;
    ORDERS_COUNT.save(storage, &val)?;
    Ok(val)
}


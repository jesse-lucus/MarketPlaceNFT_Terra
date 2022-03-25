use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ StdResult, Storage, Addr };
use cw_storage_plus::{ Map };
use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Order {
    pub asset_id: String,
    pub nft_address: Addr,
    pub seller: Addr,
    pub price: Uint128,
    pub expire_at: Uint128
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Bid {
    pub asset_id: String,
    pub nft_address: Addr,
    pub seller: Addr,
    pub bider: Addr,
    pub price: Uint128,
    pub expire_at: Uint128
}

pub const ORDERS: Map<(&str, &str), Order> = Map::new("orders");
pub const BIDS: Map<(&str, &str), Order> = Map::new("bids");


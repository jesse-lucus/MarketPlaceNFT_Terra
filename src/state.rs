use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ StdResult, Storage, Addr };
use cw_storage_plus::{ Map };
use cosmwasm_std::Uint128;
use cw0::Expiration;
use crate::asset::{Asset, AssetInfo};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Order {
    pub token_id: String,
    pub nft_address: Addr,
    pub seller: Addr,
    pub price: Asset,
    pub expire_at: Expiration
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Bid {
    pub token_id: String,
    pub nft_address: Addr,
    pub seller: Addr,
    pub bidder: Addr,
    pub price: Asset,
    pub expire_at: Expiration
}

pub const ORDERS: Map<(&str, &str), Order> = Map::new("orders");
pub const BIDS: Map<(&str, &str), Bid> = Map::new("bids");


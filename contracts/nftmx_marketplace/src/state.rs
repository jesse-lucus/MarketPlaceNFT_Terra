use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ Addr };
use cw_storage_plus::{ Map, Item };
use cosmwasm_std::{ Decimal };
use cw0::Expiration;
use crate::asset::{Asset};

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

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct Config {
  pub owner: Addr,
  pub accepted_token: Addr,
  pub owner_cut_rate: Decimal,
  pub owner_cut_rate_max: Decimal,
  pub paused: bool
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ORDERS: Map<(&str, &str), Order> = Map::new("orders");
pub const BIDS: Map<(&str, &str), Bid> = Map::new("bids");

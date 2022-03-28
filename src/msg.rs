use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use cw0::Expiration;


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderMsg {
    pub nft_address: String,
    pub asset_id: String, 
    pub price: Uint128,
    pub expire_at: Uint128
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateOrder { asset_id:String, nft_address: String, price: Uint128, expire_at: Expiration },
    CreateBid { asset_id:String, nft_address: String, price: Uint128, expire_at: Expiration },
    CancelOrder { asset_id:String, nft_address: String },
    CancelBid { asset_id:String, nft_address: String },
    ExecuteOrder { asset_id:String, nft_address: String, buyer: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ValidOrder { asset_id: String, nft_address: String },
}
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ Uint128, Decimal };
use cw0::Expiration;
use crate::asset::{Asset, AssetInfo};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: Uint128,
    pub accepted_token: String,
    pub owner_cut_rate: Decimal
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderMsg {
    pub nft_address: String,
    pub token_id: String, 
    pub price: Uint128,
    pub expire_at: Uint128
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPaused { paused: bool },
    CreateOrder { token_id:String, nft_address: String, price: Asset, expire_at: Expiration },
    UpdateOrder { token_id:String, nft_address: String, price: Asset, expire_at: Expiration },
    CreateBid { token_id:String, nft_address: String, price: Asset, expire_at: Expiration },
    CancelOrder { token_id:String, nft_address: String },
    CancelBid { token_id:String, nft_address: String },
    SafeExecuteOrder { token_id:String, nft_address: String, price: Asset },
    AcceptBid { token_id:String, nft_address: String, price: Asset }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ValidOrder { token_id: String, nft_address: String },
    ValidBid { token_id: String, nft_address: String },
    Version {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

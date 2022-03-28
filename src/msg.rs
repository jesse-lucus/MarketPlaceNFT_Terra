use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use cw0::Expiration;
use crate::asset::{Asset, AssetInfo};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: Uint128,
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
    CreateOrder { token_id:String, nft_address: String, price: Asset, expire_at: Expiration },
    CreateBid { token_id:String, nft_address: String, price: Asset, expire_at: Expiration },
    CancelOrder { token_id:String, nft_address: String },
    CancelBid { token_id:String, nft_address: String },
    ExecuteOrder { token_id:String, nft_address: String, buyer: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ValidOrder { token_id: String, nft_address: String },
    ValidBid { token_id: String, nft_address: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

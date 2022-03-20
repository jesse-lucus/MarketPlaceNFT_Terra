use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ Addr };


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OrderMsg {
    pub nft_address: Addr,
    pub token_id: String, 
    pub price: u128,
    pub expire_at: u128
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateOrder(OrderMsg)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
}
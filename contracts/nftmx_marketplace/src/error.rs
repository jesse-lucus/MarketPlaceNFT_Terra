use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::Uint128;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Given expiration is already expired or order is already expired")]
    Expired {},

    #[error("You must bid higher or equal to {} (min bid amount)", min_bid_amount)]
    MinPrice { min_bid_amount: Uint128 },  

    #[error("no bid for order")]
    NoBid {},
}

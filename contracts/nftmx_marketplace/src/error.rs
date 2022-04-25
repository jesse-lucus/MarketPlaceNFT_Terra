use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::Uint128;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Marketplace: paused")]
    MarketplacePaused {},       

    #[error("Given expiration is already expired or order is already expired")]
    Expired {},

    #[error("You must bid higher or equal to {} (min bid amount)", min_bid_amount)]
    MinPrice { min_bid_amount: Uint128 },  

    #[error("no existing order the nft and token id")]
    NoOrder {},

    #[error("no bid for order")]
    NoBid {},

    #[error("Marketplace: Only the asset owner can create orders")]
    NoOwner {},   
    
    #[error("Marketplace: Price should be bigger than 0")]
    InvalidPrice {},    
    
    #[error("Marketplace: Publication should be more than 1 minute in the future")]
    InvalidExpiration {},       

    #[error("Marketplace: bid should be > 0")]
    ZeroBidAmount {},       

    #[error("Marketplace: bid price should be higher than last bid")]
    InvalidBidAmount {},       

    #[error("Marketplace: the bid expired")]
    BidExpired {},
}

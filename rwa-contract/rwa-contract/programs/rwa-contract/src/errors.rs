use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Insufficient permissions for this action")]
    InsufficientPermissions,
    #[msg("Token is not active")]
    TokenNotActive,
    #[msg("Industry is not active")]
    IndustryNotActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
    #[msg("Auction has expired")]
    AuctionExpired,
    #[msg("Insufficient tokens available in auction")]
    InsufficientTokensAvailable,
}

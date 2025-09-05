use anchor_lang::prelude::*;
use crate::state::ComplianceStatus;

#[event]
pub struct CarbonCreditsMinted {
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct IndustryOnboarded {
    pub industry: Pubkey,
    pub company_name: String,
    pub bond_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct EmissionsReported {
    pub industry: Pubkey,
    pub co2_tonnes: u64,
    pub credits_burned: u64,
    pub reporting_period: String,
    pub compliance_status: ComplianceStatus,
    pub timestamp: i64,
}

#[event]
pub struct DutchAuctionCreated {
    pub auction: Pubkey,
    pub seller: Pubkey,
    pub start_price: u64,
    pub end_price: u64,
    pub duration_seconds: i64,
    pub tokens_for_sale: u64,
    pub timestamp: i64,
}

#[event]
pub struct BidPlaced {
    pub auction: Pubkey,
    pub bidder: Pubkey,
    pub token_amount: u64,
    pub price_per_token: u64,
    pub total_cost: u64,
    pub timestamp: i64,
}

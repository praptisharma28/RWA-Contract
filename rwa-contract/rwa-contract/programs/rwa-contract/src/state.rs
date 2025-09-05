use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub default_admin: Pubkey,
    pub bump: u8,
}

#[account]
pub struct UserRole {
    pub controller: Pubkey,
    pub role_name: String,
    pub actions: Vec<String>,
    pub users: Vec<Pubkey>,
    pub bump: u8,
}

#[account]
pub struct CarbonToken {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub co2_tonnes: u64,
    pub project_id: String,
    pub expiry_date: i64,
    pub issuer_name: String,
    pub total_supply: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
pub struct Industry {
    pub authority: Pubkey,
    pub company_name: String,
    pub registration_number: String,
    pub bond_amount: u64,
    pub is_kyc_verified: bool,
    pub is_active: bool,
    pub total_emissions: u64,
    pub credits_burned: u64,
    pub compliance_status: ComplianceStatus,
    pub onboarding_date: i64,
    pub bump: u8,
}

#[account]
pub struct DutchAuction {
    pub seller: Pubkey,
    pub token_mint: Pubkey,
    pub start_price: u64,
    pub end_price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub tokens_for_sale: u64,
    pub tokens_sold: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Frozen,
}

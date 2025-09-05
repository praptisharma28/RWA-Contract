use anchor_lang::prelude::*;

mod access_control;
mod carbon_credits;
mod industry;
mod auction;
mod state;
mod events;
mod errors;

use access_control::*;
use carbon_credits::*;
use industry::*;
use auction::*;
use state::*;
use events::*;
use errors::*;

declare_id!("4gBj3avgtDybWri9xiDQt7D3yaTiz3KbUysbVKq8Fcd4");

#[program]
pub mod carbon_rwa {
    use super::*;

    // Access Control Instructions
    pub fn initialize_access_control(
        ctx: Context<InitializeAccessControl>,
        default_admin: Pubkey,
    ) -> Result<()> {
        access_control::initialize_access_control(ctx, default_admin)
    }

    pub fn create_role(
        ctx: Context<CreateRole>,
        role_name: String,
        actions: Vec<String>,
    ) -> Result<()> {
        access_control::create_role(ctx, role_name, actions)
    }

    pub fn assign_user_to_role(
        ctx: Context<AssignUserToRole>,
        user: Pubkey,
    ) -> Result<()> {
        access_control::assign_user_to_role(ctx, user)
    }

    // Carbon Credits Instructions
    pub fn initialize_carbon_token(
        ctx: Context<InitializeCarbonToken>,
        name: String,
        symbol: String,
        uri: String,
        co2_tonnes: u64,
        project_id: String,
        expiry_date: i64,
        issuer_name: String,
    ) -> Result<()> {
        carbon_credits::initialize_carbon_token(
            ctx,
            name,
            symbol,
            uri,
            co2_tonnes,
            project_id,
            expiry_date,
            issuer_name,
        )
    }

    pub fn mint_carbon_credits(
        ctx: Context<MintCarbonCredits>,
        amount: u64,
    ) -> Result<()> {
        carbon_credits::mint_carbon_credits(ctx, amount)
    }

    // Industry Instructions
    pub fn onboard_industry(
        ctx: Context<OnboardIndustry>,
        company_name: String,
        registration_number: String,
        bond_amount: u64,
    ) -> Result<()> {
        industry::onboard_industry(ctx, company_name, registration_number, bond_amount)
    }

    pub fn report_emissions(
        ctx: Context<ReportEmissions>,
        co2_tonnes: u64,
        reporting_period: String,
    ) -> Result<()> {
        industry::report_emissions(ctx, co2_tonnes, reporting_period)
    }

    // Auction Instructions
    pub fn create_dutch_auction(
        ctx: Context<CreateDutchAuction>,
        start_price: u64,
        end_price: u64,
        duration_seconds: i64,
        tokens_for_sale: u64,
    ) -> Result<()> {
        auction::create_dutch_auction(ctx, start_price, end_price, duration_seconds, tokens_for_sale)
    }

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        token_amount: u64,
    ) -> Result<()> {
        auction::place_bid(ctx, token_amount)
    }
}

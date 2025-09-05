use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022, TokenAccount, Mint};
use anchor_spl::token_interface::{TokenInterface, MintTo, mint_to};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::*;
use crate::errors::ErrorCode;
use crate::events::CarbonCreditsMinted;
use crate::access_control::has_role;

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
    require!(
        has_role(&ctx.accounts.mint_authority_role, &ctx.accounts.authority.key(), "MINT_AUTHORITY"),
        ErrorCode::InsufficientPermissions
    );

    let carbon_token = &mut ctx.accounts.carbon_token;
    carbon_token.mint = ctx.accounts.mint.key();
    carbon_token.name = name;
    carbon_token.symbol = symbol;
    carbon_token.uri = uri;
    carbon_token.co2_tonnes = co2_tonnes;
    carbon_token.project_id = project_id;
    carbon_token.expiry_date = expiry_date;
    carbon_token.issuer_name = issuer_name;
    carbon_token.total_supply = 0;
    carbon_token.is_active = true;
    carbon_token.bump = ctx.bumps.carbon_token;

    Ok(())
}

pub fn mint_carbon_credits(
    ctx: Context<MintCarbonCredits>,
    amount: u64,
) -> Result<()> {
    // Check if user has MINT_AUTHORITY role
    require!(
        has_role(&ctx.accounts.mint_authority_role, &ctx.accounts.mint_authority.key(), "MINT_AUTHORITY"),
        ErrorCode::InsufficientPermissions
    );

    let carbon_token = &mut ctx.accounts.carbon_token;
    require!(carbon_token.is_active, ErrorCode::TokenNotActive);

    carbon_token.total_supply = carbon_token.total_supply.checked_add(amount).unwrap();

    let mint_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        mint_accounts,
    );

    mint_to(cpi_ctx, amount)?;

    emit!(CarbonCreditsMinted {
        mint: ctx.accounts.mint.key(),
        recipient: ctx.accounts.recipient.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, symbol: String)]
pub struct InitializeCarbonToken<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + name.len() + 4 + symbol.len() + 4 + 200 + 8 + 4 + 50 + 8 + 4 + 100 + 8 + 1 + 1,
        seeds = [b"carbon_token", mint.key().as_ref()],
        bump
    )]
    pub carbon_token: Account<'info, CarbonToken>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        seeds = [b"user_role", b"MINT_AUTHORITY"],
        bump = mint_authority_role.bump
    )]
    pub mint_authority_role: Account<'info, UserRole>,
    
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintCarbonCredits<'info> {
    #[account(
        mut,
        seeds = [b"carbon_token", mint.key().as_ref()],
        bump = carbon_token.bump
    )]
    pub carbon_token: Account<'info, CarbonToken>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"user_role", b"MINT_AUTHORITY"],
        bump = mint_authority_role.bump
    )]
    pub mint_authority_role: Account<'info, UserRole>,
    
    /// CHECK: This is the recipient of the minted tokens
    pub recipient: AccountInfo<'info>,
    
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}
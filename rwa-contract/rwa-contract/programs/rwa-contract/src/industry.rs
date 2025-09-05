use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::state::*;
use crate::errors::ErrorCode;
use crate::events::{IndustryOnboarded, EmissionsReported};
use crate::access_control::has_role;

pub fn onboard_industry(
    ctx: Context<OnboardIndustry>,
    company_name: String,
    registration_number: String,
    bond_amount: u64,
) -> Result<()> {
    require!(
        has_role(&ctx.accounts.kyc_authority_role, &ctx.accounts.authority.key(), "KYC_AUTHORITY"),
        ErrorCode::InsufficientPermissions
    );

    let industry = &mut ctx.accounts.industry;
    industry.authority = ctx.accounts.industry_authority.key();
    industry.company_name = company_name;
    industry.registration_number = registration_number;
    industry.bond_amount = bond_amount;
    industry.is_kyc_verified = true;
    industry.is_active = true;
    industry.total_emissions = 0;
    industry.credits_burned = 0;
    industry.compliance_status = ComplianceStatus::Compliant;
    industry.onboarding_date = Clock::get()?.unix_timestamp;
    industry.bump = ctx.bumps.industry;

    emit!(IndustryOnboarded {
        industry: ctx.accounts.industry_authority.key(),
        company_name: industry.company_name.clone(),
        bond_amount,
        timestamp: industry.onboarding_date,
    });

    Ok(())
}

pub fn report_emissions(
    ctx: Context<ReportEmissions>,
    co2_tonnes: u64,
    reporting_period: String,
) -> Result<()> {
    let industry = &mut ctx.accounts.industry;
    require!(industry.is_active, ErrorCode::IndustryNotActive);
    require!(industry.authority == ctx.accounts.industry_authority.key(), ErrorCode::Unauthorized);

    industry.total_emissions = industry.total_emissions.checked_add(co2_tonnes).unwrap();

    let current_balance = ctx.accounts.industry_token_account.amount;
    
    if current_balance >= co2_tonnes {
        industry.credits_burned = industry.credits_burned.checked_add(co2_tonnes).unwrap();
        
        emit!(EmissionsReported {
            industry: ctx.accounts.industry_authority.key(),
            co2_tonnes,
            credits_burned: co2_tonnes,
            reporting_period,
            compliance_status: ComplianceStatus::Compliant,
            timestamp: Clock::get()?.unix_timestamp,
        });
    } else {
        industry.compliance_status = ComplianceStatus::NonCompliant;
        
        emit!(EmissionsReported {
            industry: ctx.accounts.industry_authority.key(),
            co2_tonnes,
            credits_burned: current_balance,
            reporting_period,
            compliance_status: ComplianceStatus::NonCompliant,
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(company_name: String, registration_number: String)]
pub struct OnboardIndustry<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + company_name.len() + 4 + registration_number.len() + 8 + 1 + 1 + 8 + 8 + 1 + 8 + 1,
        seeds = [b"industry", industry_authority.key().as_ref()],
        bump
    )]
    pub industry: Account<'info, Industry>,
    
    #[account(
        seeds = [b"user_role", b"KYC_AUTHORITY"],
        bump = kyc_authority_role.bump
    )]
    pub kyc_authority_role: Account<'info, UserRole>,
    
    /// CHECK: This is the industry's authority public key
    pub industry_authority: AccountInfo<'info>,
    
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportEmissions<'info> {
    #[account(
        mut,
        seeds = [b"industry", industry_authority.key().as_ref()],
        bump = industry.bump
    )]
    pub industry: Account<'info, Industry>,
    
    #[account(
        token::mint = token_mint,
        token::authority = industry_authority,
        token::token_program = token_program,
    )]
    pub industry_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// CHECK: This is the token mint for carbon credits
    pub token_mint: AccountInfo<'info>,
    
    pub industry_authority: Signer<'info>,
    
    /// CHECK: This account is used to specify the token program interface
    pub token_program: AccountInfo<'info>,
}

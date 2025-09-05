use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn initialize_access_control(
    ctx: Context<InitializeAccessControl>,
    default_admin: Pubkey,
) -> Result<()> {
    let controller = &mut ctx.accounts.controller;
    controller.default_admin = default_admin;
    controller.bump = ctx.bumps.controller;
    Ok(())
}

pub fn create_role(
    ctx: Context<CreateRole>,
    role_name: String,
    actions: Vec<String>,
) -> Result<()> {
    require!(ctx.accounts.controller.default_admin == ctx.accounts.admin.key(), ErrorCode::Unauthorized);
    
    let user_role = &mut ctx.accounts.user_role;
    user_role.controller = ctx.accounts.controller.key();
    user_role.role_name = role_name;
    user_role.actions = actions;
    user_role.users = Vec::new();
    user_role.bump = ctx.bumps.user_role;
    Ok(())
}

pub fn assign_user_to_role(
    ctx: Context<AssignUserToRole>,
    user: Pubkey,
) -> Result<()> {
    require!(ctx.accounts.controller.default_admin == ctx.accounts.admin.key(), ErrorCode::Unauthorized);
    
    let user_role = &mut ctx.accounts.user_role;
    if !user_role.users.contains(&user) {
        user_role.users.push(user);
    }
    Ok(())
}

pub fn has_role(user_role_account: &Account<UserRole>, user: &Pubkey, required_action: &str) -> bool {
    user_role_account.actions.contains(&required_action.to_string()) && 
    user_role_account.users.contains(user)
}

#[derive(Accounts)]
pub struct InitializeAccessControl<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 1,
        seeds = [b"controller"],
        bump
    )]
    pub controller: Account<'info, Controller>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(role_name: String)]
pub struct CreateRole<'info> {
    #[account(
        seeds = [b"controller"],
        bump = controller.bump
    )]
    pub controller: Account<'info, Controller>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + role_name.len() + 4 + (32 * 10) + 4 + (32 * 100) + 1,
        seeds = [b"user_role", role_name.as_bytes()],
        bump
    )]
    pub user_role: Account<'info, UserRole>,
    
    pub admin: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignUserToRole<'info> {
    #[account(
        seeds = [b"controller"],
        bump = controller.bump
    )]
    pub controller: Account<'info, Controller>,
    
    #[account(
        mut,
        seeds = [b"user_role", user_role.role_name.as_bytes()],
        bump = user_role.bump
    )]
    pub user_role: Account<'info, UserRole>,
    
    pub admin: Signer<'info>,
}

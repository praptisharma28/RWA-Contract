use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::ErrorCode;



pub fn create_dutch_auction(
    ctx: Context<ContextDutchAuction>,
    start_price: u64,
    end_price: u64,
    duration_seconds: i64,
    tokens_for_sale: u64,
) -> Result<()>{
    require!(has_role(&ctx.accounts.auction_authority_role, &ct.accounts.authority.key(), "AUCTION_AUTHORITY"),
    ErrorCode::
    );
    let auction = &mut ctx.accounts.auction;
    let current_time = Clock::get()?.unix_timestamp;
    auction.seller = ctx.accounts.seller.key();
    auction.token_mint = ctx.accounts.token_mint.key();
    auction.start_price = start_price;
    auction.end_price= end_price;
    auction.start_time= current_time;
    auction.end_time= current_time+duration_seconds;
    auction.tokens_for_sale= tokens_for_sale;
    auction.tokens_sold= 0;
    auction.is_active = true;
    auction.bump = ctx.bumps.auction;
    emit!(DutchAuctionCreated{
        auction: auction.key(),
        seller: auction.seller,
        start_price,
        end_price,
        duration_seconds,
        tokens_for_sale,
        timestamp: current_time,
    });
    Ok(())
}


pub fn place_bid(
    ctx: Context<PlaceBid>,
    token_amount: u64,
) -> Result<()>{
    let auction = &mut ctx.accounts.auction;
    let current_time= Clock::get()?.unix_timestamp;
    require!(auction.is_active , ErrorCode::);
    require!(current_time <= auction.end_time, ErrorCode);
    require!(token_amount <= auction.tokens_for_sale - auction.tokens_sold, ErrorCode);
    let current_price = calculate_dutch_auction_price(
        auction.start_price,
        auction.end_price,
        auction.start_time,
        auction.end_time,
        current_time,
    );

    let total_cost = current_price.checked_mul(token_account).unwrap();
    auction.token_sold = auction.token_sold.checked_add(token_account).unwrap();
    if auction.token_sold >= auction.tokens_for_sale{
        auction.is_active = false;
    }

    emit!(BidPlaced{
    auction: auction.key(),
    bidder: ctx.accounts.bidder.key(),
    token_amount,
    price_per_token: current_price,
    total_cost,
    timestamp: current_time,
    });
    Ok(())
}

pub fn calculate_dutch_auction_price(
    start_price: u64,
    end_price: u64,
    start_time: i64,
    end_time: i64,
    current_time: i64,
) -> u64{
    if current_time >= end_time{
        return end_price;
    }
    let total_duration = end_time- start_time;
    let elapsed_time = current_time- start_time;
    let price_decay= (start_price-end_price) * elapsed_time  as u64/ total_duration as u64;
    start_price - price_decay 
}

#[derive(Accounts)]
pub struct CreateDutchAuction<'info>{
    #[account(
        init,
        payer = payer,
        space =
        seeds = [b"dutch_auction", seller.key().as_ref(), token_mint.key().as_bytes()],
        bump
    )]
    pub auction: Account<'info, DutchAuction>,

    #[account(
        seeds= [b"user_role", b"AUCTION_AUTHORITY"],
        bump = auction_authority_role.bump
    )]
    pub auction_authority_role: Account<'info, UserRole>,
    pub seller: AccountInfo<'info>,
    pub token_mint: AccountInfo<'info>,
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(
        mut,
        seeds= [b"dutch_auction", auction_seller.as_ref(), auction.token_mint.as_ref()],
        bump = auction.bump
    )]
    pub auction: Account<'info, DutchAuction>,
    pub bidder: Signer<'info>,
}
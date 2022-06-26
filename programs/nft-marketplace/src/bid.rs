use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::token,
};


pub fn bid(
    ctx: Context<BidNft>,
    bid_lamports: u64,
) -> Result<()> {

    msg!("Program ID: {}", &ctx.program_id);

    let bid_account = &mut ctx.accounts.bid_account;
    bid_account.bid += bid_lamports;
    bid_account.public_key = ctx.accounts.authority.key();
    
    msg!("Bid submitted!");

    Ok(())
}


#[derive(Accounts)]
pub struct BidNft<'info> {
    #[account(zero)]
    pub mint: Account<'info, token::Mint>,
    #[account(
        init,
        payer = authority,
        space = 256,
        seeds = [
            b"bid_",
            mint.key().as_ref(),
        ],
        bump
    )]
    pub bid_account: Account<'info, Bid>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bid {
    pub bid: u64,
    pub public_key: Pubkey,
}
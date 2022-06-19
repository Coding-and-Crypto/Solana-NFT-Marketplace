use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::token,
};


pub fn sell(
    ctx: Context<SellNft>,
    sale_lamports: u64
) -> Result<()> {

    msg!(
        "Received request to sell NFT {} for {} lamports.", 
        &ctx.accounts.mint.key(),
        sale_lamports,
    );
    msg!("Owner: {}", &ctx.accounts.owner_authority.key());
    msg!("Purchaser: {}", &ctx.accounts.buyer_authority.key());

    // Transfer SOL from buyer to owner
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer_authority.to_account_info(),
                to: ctx.accounts.owner_authority.to_account_info(),
            }
        ),
        sale_lamports
    )?;
    msg!(
        "{} lamports transferred from {} to {} successfully.",
        sale_lamports,
        &ctx.accounts.buyer_authority.key(),
        &ctx.accounts.owner_authority.key(),
    );

    // Transfer NFT from owner to buyer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.owner_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!(
        "NFT {} transferred from {} to {} successfully.",
        sale_lamports,
        &ctx.accounts.owner_token_account.key(),
        &ctx.accounts.buyer_token_account.key(),
    );
    
    msg!("Sale completed!");

    Ok(())
}


#[derive(Accounts)]
pub struct SellNft<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    // pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            native_token::LAMPORTS_PER_SOL,
            program::invoke,
            system_instruction::create_account,
        },
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::instruction::{
        create_master_edition_v3, create_metadata_accounts_v2
    },
};

pub fn mint(
    ctx: Context<MintNft>, 
    metadata_title: String, 
    metadata_symbol: String, 
    metadata_uri: String,
) -> Result<()> {

    msg!("Initializing...");

    // Create Mint
    let mint = ctx.accounts.mint.to_account_info();
    msg!("Token generated: {}", &mint.key());

    // Create Associated Token Account

    msg!("Token mint process completed.");

    Ok(())
}


#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 40,
    )]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
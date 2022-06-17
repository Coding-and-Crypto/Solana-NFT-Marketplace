use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            native_token::LAMPORTS_PER_SOL,
            program::invoke,
        },
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction,
        state,
    },
};

pub fn mint(
    ctx: Context<MintNft>, 
    metadata_title: String, 
    metadata_symbol: String, 
    metadata_uri: String,
) -> Result<()> {

    msg!("Minting token...");
    msg!("Token: {}", &ctx.accounts.mint.to_account_info().key());    
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo{
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Creating metadata...");
    msg!("Metadata address: {}", &ctx.accounts.metadata.to_account_info().key());
    invoke(
        &instruction::create_metadata_accounts_v2(
            TOKEN_METADATA_ID, 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            metadata_title, 
            metadata_symbol, 
            metadata_uri, 
            None,
            1,
            true, 
            false, 
            None, 
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Creating master edition metadata...");
    msg!("Master edition address: {}", &ctx.accounts.master_edition.to_account_info().key());
    invoke(
        &instruction::create_master_edition_v3(
            TOKEN_METADATA_ID, 
            ctx.accounts.master_edition.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint_authority.key(), 
            Some(0),
        ),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Token mint process completed.");

    Ok(())
}


#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}
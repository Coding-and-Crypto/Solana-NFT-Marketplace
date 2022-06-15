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
        // system_program::create_account(
        //     CpiContext::new(
        //         ctx.accounts.mint.to_account_info(),
        //         system_program::CreateAccount {
        //             from: ctx.accounts.payer.to_account_info(),
        //             to: ctx.accounts.mint.to_account_info(),
        //         },
        //     ),
        //     LAMPORTS_PER_SOL,
        //     48,
        //     &ctx.accounts.token_program.key(),
        // )?;
        // msg!("Token generated: {}", &ctx.accounts.mint.key());

        // Initialize Mint
        // token::initialize_mint(
        //     CpiContext::new(
        //         ctx.accounts.mint.to_account_info(),
        //         token::InitializeMint {
        //             mint: ctx.accounts.mint.to_account_info(),
        //             rent: ctx.accounts.rent.to_account_info(),
        //         },
        //     ),
        //     0,
        //     &ctx.accounts.mint_authority.key(),
        //     None,
        // )?;
        // msg!("Mint initialized: {}", &ctx.accounts.mint.key());

        // // Create associated token account
        // associated_token::create(
        //     CpiContext::new(
        //         ctx.accounts.mint.to_account_info(),
        //         associated_token::Create {
        //             associated_token: ctx.accounts.mint.to_account_info(),
        //             authority: ctx.accounts.payer.to_account_info(),
        //             mint: ctx.accounts.mint.to_account_info(),
        //             payer: ctx.accounts.payer.to_account_info(),
        //             rent: ctx.accounts.rent.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token_program.to_account_info(),
        //         },
        //     )
        // )?;
        // msg!("Associated token account created.");
        // msg!("{}", &ctx.accounts.token_account.key());

        // Mint to
        token::mint_to(
            CpiContext::new(
                ctx.accounts.mint.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1
        )?;
        msg!("Token minted to associated token address.");

        // invoke(
        //     &create_metadata_accounts_v2(
        //         ctx.accounts.token_metadata_program.key(),
        //         ctx.accounts.metadata.key(),
        //         ctx.accounts.mint.key(),
        //         ctx.accounts.mint_authority.key(),
        //         ctx.accounts.payer.key(),
        //         ctx.accounts.payer.key(),
        //         metadata_title,
        //         metadata_symbol,
        //         metadata_uri,
        //         None,
        //         1,
        //         true,
        //         false,
        //         None,
        //         None,
        //     ),
        //     &[
        //         ctx.accounts.metadata.to_account_info(),
        //         ctx.accounts.mint.to_account_info(),
        //         ctx.accounts.mint_authority.to_account_info(),
        //         ctx.accounts.payer.to_account_info(),
        //         ctx.accounts.token_metadata_program.to_account_info(),
        //         ctx.accounts.token_program.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //         ctx.accounts.rent.to_account_info(),
        //     ]
        // )?;
        // msg!("Metadata accounts created.");

        // invoke(
        //     &create_master_edition_v3(
        //         ctx.accounts.token_metadata_program.key(),
        //         ctx.accounts.master_edition.key(),
        //         ctx.accounts.mint.key(),
        //         ctx.accounts.payer.key(),
        //         ctx.accounts.mint_authority.key(),
        //         ctx.accounts.metadata.key(),
        //         ctx.accounts.payer.key(),
        //         Some(0),
        //     ),
        //     &[
        //         ctx.accounts.master_edition.to_account_info(),
        //         ctx.accounts.mint.to_account_info(),
        //         ctx.accounts.mint_authority.to_account_info(),
        //         ctx.accounts.payer.to_account_info(),
        //         ctx.accounts.metadata.to_account_info(),
        //         ctx.accounts.token_metadata_program.to_account_info(),
        //         ctx.accounts.token_program.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //         ctx.accounts.rent.to_account_info(),
        //     ]
        // )?;
        // msg!("Master edition created.");

        msg!("Token mint process completed.");

        Ok(())
}


#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub master_edition: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub metadata: Account<'info, Metadata>,
    #[account(init, payer = mint_authority, space = 8 + 40)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    #[account(init, payer = mint_authority, space = 8 + 40)]
    pub token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub token_metadata_program: Program<'info, token::Token>,
}

#[account]
pub struct Metadata {
    name: String,
    symbol: String,
    description: String,
    seller_fee_basis_points: u32,
    external_url: String,
    edition: String,
    background_color: String,
    image: String,
}
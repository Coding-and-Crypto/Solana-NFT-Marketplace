use anchor_lang::prelude::*;

pub mod bid;
pub mod mint;
pub mod sell;

use bid::*;
use mint::*;
use sell::*;


declare_id!("H2UJjAQTuVJYhaBhh6GD2KaprLBTp1vhP2aaHioya5NM");


#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {
        mint::mint(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )
    }

    pub fn bid(
        ctx: Context<Bid>,
        lamports: u64,
    ) -> Result<()> {
        bid::bid(
            ctx,
            lamports,
        )
    }

    pub fn sell(
        ctx: Context<Sell>,
    ) -> Result<()> {
        sell::sell(
            ctx,
        )
    }
}
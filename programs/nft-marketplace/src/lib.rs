use anchor_lang::prelude::*;

pub mod bid;
pub mod mint;
pub mod sell;

use bid::*;
use mint::*;
use sell::*;


declare_id!("4PYm9iCSmNQokAATfDdAsHzQsiXkcCSNTUzQU8MisNTK");


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
        ctx: Context<BidNft>,
        bid_lamports: u64,
    ) -> Result<()> {
        bid::bid(
            ctx,
            bid_lamports,
        )
    }

    pub fn sell(
        ctx: Context<SellNft>,
        sale_lamports: u64
    ) -> Result<()> {
        sell::sell(
            ctx,
            sale_lamports,
        )
    }
}
use anchor_lang::prelude::*;


pub fn bid(
    ctx: Context<Bid>,
    lamports: u64,
) -> Result<()> {
    
    msg!("Bid submitted!");

    Ok(())
}


#[derive(Accounts)]
pub struct Bid {}
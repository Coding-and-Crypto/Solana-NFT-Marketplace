use anchor_lang::prelude::*;


pub fn sell(
    ctx: Context<Sell>,
) -> Result<()> {
    
    msg!("Sale completed!");

    Ok(())
}


#[derive(Accounts)]
pub struct Sell {}
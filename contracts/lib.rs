use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod cornhole {
    use super::*;
    pub fn toss(ctx: Context<Toss>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Toss {}

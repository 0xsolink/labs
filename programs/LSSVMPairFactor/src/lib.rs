use anchor_lang::prelude::*;

declare_id!("7Ra2VuCxJcZpaizvHp9DYboVStmeaUMq9VWEPvjKs1g");

#[program]
pub mod lssvm_pair_factor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_1(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
   
}

#[derive(Accounts)]
pub struct Initialize {}

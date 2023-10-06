use anchor_lang::prelude::*;

declare_id!("HpSRXbwyB1wtiZGhWg1ce6yDXjgyZ523cQ5PyXMEm7zw");

#[program]
pub mod lssvm_pair {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

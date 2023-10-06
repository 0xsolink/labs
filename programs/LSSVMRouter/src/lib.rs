use anchor_lang::prelude::*;

declare_id!("B7PeT5iTQUmFpoufP2gkzACWi4SRZTCjgJYF4sP9H1qf");

#[program]
pub mod lssvm_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

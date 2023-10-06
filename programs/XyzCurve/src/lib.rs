use anchor_lang::prelude::*;

declare_id!("ETxjHW9By3eWEet7wWgkNTja4WRWeWptzyFKL42H3TkN");

#[program]
pub mod xyz_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

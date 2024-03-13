mod errors;
mod contexts;
mod state;

use anchor_lang::prelude::*;

declare_id!("KXnRjPHroM6rJTeVy51JPkiBuS5vD87VUdB5a17GPDK");

#[program]
pub mod cade_economy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

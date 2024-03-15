use anchor_lang::prelude::*;

pub mod state;

pub use state::*;

pub mod contexts;

pub use contexts::*;
// mod helpers;

pub mod errors;

pub use errors::AmmError;

declare_id!("KXnRjPHroM6rJTeVy51JPkiBuS5vD87VUdB5a17GPDK");

#[program]
pub mod newamm {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

}
use anchor_lang::prelude::*;

pub mod state;

pub use state::*;

pub mod contexts;

pub use contexts::*;
// mod helpers;

pub mod errors;

pub use errors::AmmError;

declare_id!("FtrbVfeTkte7b9KTHYzpaRzJZT1t1SHaR1QuhyAusNTW");

#[program]
pub mod newamm {
    use crate::instruction::{ClaimUsdcForCade, LpInitialize, Pay};
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

    pub fn lp_initialize(
        ctx: Context<InitializeLP>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

    pub fn mint_lp(ctx: Context<MintCade>, amount: u64) -> Result<()> {
        ctx.accounts.mint_lp(amount)
    }

    pub fn swap(ctx: Context<Swap>, amount: u64, expiration: i64) -> Result<()> {
        ctx.accounts.swap(amount, expiration)
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        ctx.accounts.pay(amount)
    }

    pub fn claim_usdc_for_cade(ctx: Context<ClaimUsdcForCade>) -> Result<()> {
        ctx.accounts.claim_usdc()
    }
}

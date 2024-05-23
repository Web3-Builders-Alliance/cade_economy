use anchor_lang::prelude::*;

pub mod state;

pub use state::*;

pub mod contexts;

pub use contexts::*;
// mod helpers;

pub mod errors;

pub use errors::AmmError;

declare_id!("DeMJhDYgRpStEnPXNB9r8htoTHNeuXmAnAVoiRUvVtuq");

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

    pub fn lp_initialize(
        ctx: Context<InitializeLP>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

    pub fn bonk_initialize(
        ctx: Context<InitializeBonk>,
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


    pub fn swap_with_bonk(ctx: Context<SwapWithBonk>, amount: u64, expiration: i64) -> Result<()> {
        ctx.accounts.swap_with_bonk(amount, expiration)
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        ctx.accounts.pay(amount)
    }

    pub fn pay_with_bonk(ctx: Context<PayWithBonk>, amount: u64) -> Result<()> {
        ctx.accounts.pay_with_bonk(amount)
    }

    pub fn claim_usdc_for_cade(ctx: Context<ClaimUsdc>) -> Result<()> {
        ctx.accounts.claim_usdc()
    }
}

use anchor_lang::prelude::*;
use crate::state::PrizeConfig;
use anchor_spl::{token::{TokenAccount, Mint}, associated_token::AssociatedToken};
use anchor_spl::token::Token;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializePrize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub prize_mint: Account<'info, Mint>,
    #[account(
    init,
    payer = user,
    associated_token::mint = prize_mint,
    associated_token::authority = prize_auth
    )]
    pub particular_prize_vault: Account<'info, TokenAccount>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"prize_auth"],
    bump
    )]
    pub prize_auth: UncheckedAccount<'info>,
    #[account(
    init,
    payer = user,
    seeds = [b"prize", seed.to_le_bytes().as_ref()],
    bump,
    space = PrizeConfig::INIT_SPACE
    )]
    pub prize_config: Account<'info, PrizeConfig>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePrize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializePrizeBumps,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        self.prize_config.set_inner(
            PrizeConfig {
                seed,
                authority,
                mint_prize: self.prize_mint.key(),
                prize_bump: bumps.prize_auth,
                prize_config_bump: bumps.prize_config,
            }
        );
        Ok(())
    }
}

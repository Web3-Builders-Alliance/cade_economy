use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::Bonk_Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeBonk<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_bonk: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_bonk,
        associated_token::authority = auth
    )]
    pub vault_bonk: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"bonk_config", seed.to_le_bytes().as_ref()],
        bump,
        space = Bonk_Config::INIT_SPACE
    )]
    pub bonk_config: Box<Account<'info, Bonk_Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeBonk<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBonkBumps,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        self.bonk_config.set_inner(
            Bonk_Config {
                seed: seed,
                authority: authority,
                mint_bonk: self.mint_bonk.key(),
                bonk_config_bump: bumps.bonk_config,
            });
        Ok(())
    }
}
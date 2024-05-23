use anchor_lang::prelude::*;
use crate::state::Config;
use anchor_spl::{token_interface::{TokenAccount, Mint}, associated_token::AssociatedToken};
use anchor_spl::token_interface::TokenInterface;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user2: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer = user2,
        associated_token::mint = mint_x,
        associated_token::authority = new_auth
    )]
    pub vault_y: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
        seeds = [b"new_auth"],
        bump
    )]
    pub new_auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config: Box<Account<'info, Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBumps,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        self.config.set_inner(
            Config {
                seed: seed,
                authority: authority,
                mint_x: self.mint_x.key(),
                auth_bump: bumps.auth,
                new_auth_bump: bumps.new_auth,
                config_bump: bumps.config,
            });

        Ok(())
    }
}
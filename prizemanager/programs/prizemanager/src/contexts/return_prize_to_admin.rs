use anchor_lang::prelude::*;
use crate::state::PrizeConfig;
use anchor_spl::{token::{TokenAccount, Mint}, associated_token::AssociatedToken};
use anchor_spl::token::{Token, Transfer, transfer};

#[derive(Accounts)]
pub struct GivePrizeBackToAdmin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub prize_mint: Account<'info, Mint>,
    #[account(
    mut,
    associated_token::mint = prize_mint,
    associated_token::authority = prize_auth
    )]
    pub particular_prize_vault: Account<'info, TokenAccount>,
    #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = prize_mint,
    associated_token::authority = user
    )]
    pub admin_prize_vault: Account<'info, TokenAccount>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"prize_auth"],
    bump = prize_config.prize_bump
    )]
    pub prize_auth: UncheckedAccount<'info>,
    #[account(
    seeds = [
    b"prize",
    prize_config.seed.to_le_bytes().as_ref()
    ],
    bump = prize_config.prize_config_bump,
    )]
    pub prize_config: Account<'info, PrizeConfig>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> GivePrizeBackToAdmin<'info> {
    pub fn give_prize_back_to_admin(
        &mut self
    ) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.particular_prize_vault.to_account_info(),
            to: self.admin_prize_vault.to_account_info(),
            authority: self.prize_auth.to_account_info(),
        };
        let seeds = &[&b"prize_auth"[..], &[self.prize_config.prize_bump]];

        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds
        );

        transfer(ctx, 1)
    }
}


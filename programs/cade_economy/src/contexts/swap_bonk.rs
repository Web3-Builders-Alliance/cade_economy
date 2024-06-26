use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};
use crate::Lp_Config;
// use crate::{assert_non_zero,assert_not_expired,assert_not_locked};

#[derive(Accounts)]
pub struct SwapWithBonk<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub bonk_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = lp_config.lp_bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = auth
    )]
    pub vault_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = auth
    )]
    pub bonk_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_vault_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = bonk_mint,
        associated_token::authority = user
    )]
    pub user_vault_bonk: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
        seeds = [b"auth"],
        bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [
        b"config",
        config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        seeds = [
        b"lp_config",
        config.seed.to_le_bytes().as_ref()
        ],
        bump = lp_config.lp_config_bump,
    )]
    pub lp_config: Box<Account<'info, Lp_Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> SwapWithBonk<'info> {
    pub fn swap_with_bonk(&mut self, amount: u64, expiration: i64) -> Result<()> {
        // assert_not_locked!(self.config.locked);
        // assert_not_expired!(expiration);
        // assert_non_zero!([amount]);

        self.deposit_token(amount);
        self.send_lp_token(amount * 10);

        Ok(())
    }

    pub fn deposit_token(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.user_vault_bonk.to_account_info(),
            mint: self.bonk_mint.to_account_info(),
            to: self.bonk_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx, amount, self.bonk_mint.decimals);
        Ok(())
    }

    pub fn send_lp_token(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            to: self.user_vault_lp.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer_checked(ctx, amount, self.mint_lp.decimals)
    }
}

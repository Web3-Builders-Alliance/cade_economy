use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
         transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};
use crate::Lp_Config;
// use crate::{assert_non_zero,assert_not_expired,assert_not_locked};

#[derive(Accounts)]
pub struct ClaimUsdc<'info> {
    #[account(mut)]
    pub gamer: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    mut,
    seeds = [b"lp" , config.key().as_ref()],
    bump = lp_config.lp_bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    mut,
    associated_token::mint = mint_x,
    associated_token::authority = new_auth
    )]
    pub vault_y : Box<InterfaceAccount<'info,TokenAccount>>,
    #[account(
    mut,
    associated_token::mint = mint_lp,
    associated_token::authority = auth
    )]
    pub vault_lp : Box<InterfaceAccount<'info , TokenAccount>>,
    #[account(
    init_if_needed,
    payer = gamer,
    associated_token::mint = mint_lp,
    associated_token::authority = gamer
    )]
    pub gamer_vault_lp : Box<InterfaceAccount<'info,TokenAccount>>,
    #[account(
    init_if_needed,
    payer = gamer,
    associated_token::mint = mint_x,
    associated_token::authority = gamer
    )]
    pub gamer_vault_x : Box<InterfaceAccount<'info,TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"auth"],
    bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"new_auth"],
    bump
    )]
    pub new_auth : UncheckedAccount<'info>,
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

impl<'info> ClaimUsdc<'info> {
    pub fn claim_usdc(&mut self) -> Result<()> {
        // assert_not_locked!(self.config.locked);
        // assert_not_expired!(expiration);
        // assert_non_zero!([amount]);

        let cade_amount = self.gamer_vault_lp.amount;
        let usdc_amount_to_claim_by_gamer = cade_amount/10;

        self.send_cade_back_to_cade_vault(cade_amount);
        self.receive_usdc(usdc_amount_to_claim_by_gamer);

        Ok(())
    }

    pub fn send_cade_back_to_cade_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.gamer_vault_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            to: self.vault_lp.to_account_info(),
            authority: self.gamer.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx,amount , self.mint_x.decimals)
    }

    pub fn receive_usdc(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault_y.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.gamer_vault_x.to_account_info(),
            authority: self.new_auth.to_account_info(),
        };

        let seeds = &[&b"new_auth"[..], &[self.config.new_auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer_checked(ctx, amount, self.mint_x.decimals)
    }
}

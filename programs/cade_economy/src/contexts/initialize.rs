use anchor_lang::prelude::*;
use crate::state::Config;
use anchor_spl::{token_interface::{TokenAccount, Mint, TokenInterface}, associated_token::AssociatedToken};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user : Signer<'info>,
    pub mint_x : Box<InterfaceAccount<'info,Mint>>,
    #[account(
    init,
    payer = user,
    seeds = [b"lp" , config.key().as_ref()],
    bump,
    mint::decimals = 6,
    mint::authority = auth
    )]
    pub mint_lp : Box<InterfaceAccount<'info,Mint>>,
    #[account(
    init,
    payer = user,
    associated_token::mint = mint_x,
    associated_token::authority = auth
    )]
    pub vault_x : Box<InterfaceAccount<'info,TokenAccount>>,
    #[account(
    init,
    payer = user,
    associated_token::mint = mint_lp,
    associated_token::authority = auth
    )]
    pub vault_lp : Box<InterfaceAccount<'info , TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"auth"],
    bump
    )]
    pub auth : UncheckedAccount<'info>,
    #[account(
    init,
    payer = user,
    seeds = [b"config",seed.to_le_bytes().as_ref()],
    bump,
    space = Config::INIT_SPACE
    )]
    pub config : Box<Account<'info , Config>>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBumps,
        seed : u64,
        authority : Option<Pubkey>
    ) -> Result<()> {
        self.config.set_inner(
            Config {
                seed,
                authority,
                mint_x: self.mint_x.key(),
                mint_lp : self.mint_lp.key(),
                locked: false,
                auth_bump: bumps.auth,
                config_bump: bumps.config,
                lp_bump: bumps.mint_lp,
            });

        Ok(())
    }
}
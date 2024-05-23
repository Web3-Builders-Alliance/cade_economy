use anchor_lang::prelude::*;

#[account]
pub struct Bonk_Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_bonk: Pubkey,
    pub bonk_config_bump: u8,
}

impl Space for Bonk_Config {
    const INIT_SPACE: usize = 8 + 8 + (1 + 32) + 32 + 1;
}
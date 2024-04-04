use anchor_lang::prelude::*;

#[account]
pub struct Lp_Config {
    pub seed : u64,
    pub authority : Option<Pubkey>,
    pub mint_lp : Pubkey,
    pub lp_bump : u8,
    pub lp_config_bump : u8
}

impl Space for Lp_Config {
    const INIT_SPACE: usize = 8 + 8 + (1+32) + 32 + (2 * 1);
}
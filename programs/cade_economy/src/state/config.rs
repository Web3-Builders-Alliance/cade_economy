use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed : u64,
    pub authority : Option<Pubkey>,
    pub mint_x : Pubkey,
    pub auth_bump : u8,
    pub new_auth_bump : u8,
    pub config_bump : u8,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 8 + (1+32) + 32 + 32 + (3 * 1);
}
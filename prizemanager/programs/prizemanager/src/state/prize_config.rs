use anchor_lang::prelude::*;

#[account]
pub struct PrizeConfig {
    pub seed : u64,
    pub authority : Option<Pubkey>,
    pub mint_prize : Pubkey,
    pub prize_bump : u8,
    pub prize_config_bump : u8
}

impl Space for PrizeConfig {
    const INIT_SPACE: usize = 8 + 8 + (1+32) + 32 + (2*1);
}
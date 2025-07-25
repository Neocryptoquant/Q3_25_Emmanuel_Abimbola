use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    //pub bmp: u8,
    pub staked_at: i64, //i64 because time cannot be negative
    pub bump: u8,
}

// Map one to many:
// StakeConfige => UserAccount => StakeAccount.
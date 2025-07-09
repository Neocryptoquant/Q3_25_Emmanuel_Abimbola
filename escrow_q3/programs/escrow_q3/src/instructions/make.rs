use anchor_lang::{prelude::*, system_program::Transfer};
//use anchor_spl::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::program::EscrowQ3;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make <'info> {
    #[account(mut)]
    pub maker: Signer <'info>,
    
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount <'info, Mint>,
    
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount <'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount <'info, TokenAccount>,
    
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker, 
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface <'info, TokenInterface>,
    pub system_program: Program<'info, System>

}

impl<'info> Make <'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escroe.set_inner(
            Escrow {
                seed,
                maker: self.maker.key(),
                mint_a: self.mint_a.key(),
                mint_b: self.mint_b.key(),
                receive,
                bump: bumps.escrow
            });

        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {

        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint self.maker_ata_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: 
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info());

        transfer_checked(cpi_ctx, transfer_accounts);
    }
}
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}


// Token2022 extensions allow for extended functions and features to be added.
// It has been adopted by some stocks. Because it it institutional friendly.
// Every single lineof code is a vuln point
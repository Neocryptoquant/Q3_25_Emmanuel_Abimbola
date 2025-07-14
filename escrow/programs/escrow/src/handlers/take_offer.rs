use anchor_lang::{prelude::*};
use crate::error::ErrorCode;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount,TokenInterface}
};
use crate::state::Offer;
use crate::{transfer_tokens, close_token_account};


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TakeOffer<'info> {
    // TakeOffer (in capitals) is a struct of names accounts that the
    // take_offer() function will use.
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub taker: Signer<'info>,

    
    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    
    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program =token_program
    )]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program =token_program
    )]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,

        #[account(
        mut,
        close = taker,
        //has_one = taker,
        has_one = token_mint_b,
        seeds = [b"offer", offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        associated_token::mint = token_mint_b,
        associated_token::authority = offer,
        associated_token::token_program =token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
}

// Handle the take offer instruction by:
// 1. Sending the wanted tokens from the taker to the maker
// 2. Withdrawing the offered tokens from the vault to the taker and closing the vault
pub fn take_offer(
    context: Context<TakeOffer>,
    // token_b_wanted_amount: u64,
    //token_a_offered_amount: u64,
    // id: u64,
    // amount: u64,
    // token_b_wanted_amount: u64 
) -> Result<()> {

    let offer_account_seeds = &[
        b"offer",
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ];
    let signer_seeds = Some(&offer_account_seeds[..]);

    transfer_tokens(
        &context.accounts.vault,       
        &context.accounts.taker_token_account_b,
        &context.accounts.vault.amount,       
        &context.accounts.token_mint_a,       
        &context.accounts.offer.to_account_info(),       
        &context.accounts.token_program,
        signer_seeds,
    ).map_err(|_| error!(ErrorCode::FailedVaultWithdrawal))?;

    close_token_account(
        &context.accounts.vault,
        &context.accounts.taker.to_account_info(),
        &context.accounts.offer.to_account_info(),
        &context.accounts.token_program,
        signer_seeds,
    ).map_err(|_| error!(ErrorCode::FailedVaultClosure))?;

    //from the taker to the maker. sending the wanted tokens.
    
    transfer_tokens(
        &context.accounts.taker_token_account_b,       
        &context.accounts.maker_token_account_b,
        &context.accounts.offer.token_b_wanted_amount,       
        &context.accounts.token_mint_b,       
        &context.accounts.taker.to_account_info(),       
        &context.accounts.token_program,
        None,
    ).map_err(|_| error!(ErrorCode::InsufficientTakerBalance))?;
    Ok(())
}

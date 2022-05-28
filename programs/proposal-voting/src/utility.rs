use anchor_lang::prelude::*;

use anchor_spl::token::{TokenAccount};
use crate::error::ErrorCode;

pub struct Utility;

impl Utility {
    pub fn verify_token_account_amount(
        token_account: &TokenAccount, 
        user_key: &Pubkey, 
        token_amount: u64,
    ) -> Result<()> {

        // Make sure the user is the owner of the token account
        if token_account.owner != *user_key {
            // Log a formatted message, use with caution can be expensive
            msg!("{} key: {:?}", "token", token_account.owner);
            msg!("{} key: {:?}", "user", *user_key);
            return Err(ErrorCode::UnauthorizedTokenHolder.into());
        }

        // Check that the token balance meets the minimum required balance for voting specified by proposal
        if token_account.amount < token_amount {
            return Err(ErrorCode::InsufficientTokensToVote.into());
        }
        Ok(())
    }

    pub fn verify_token_account(
        token_account: &TokenAccount, 
        user_key: &Pubkey, 
        mint_key: &Pubkey,
        token_amount: u64,
    ) -> Result<()> {

        // Make sure the token account and mint match
        if token_account.mint != *mint_key {
            return Err(ErrorCode::InvalidTokenAccount.into());
        }

        Utility::verify_token_account_amount(token_account, user_key, token_amount)
    }
}
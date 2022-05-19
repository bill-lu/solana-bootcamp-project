use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

pub const ACCOUNT_DISCRIMINATOR_LENGTH: usize = 8;

// enums
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initialized,
    Open,
    Closed
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VoteOption {
    Approve,
    Reject,
    Abandon
}

impl Default for VoteOption {
    fn default() -> Self { VoteOption::Approve }
}
// Accounts
#[account]
pub struct Proposal {    
    pub id: u32,                // unique id for each proposal
    pub authority: Pubkey,      // account creates the proposal
    pub mint_of_token: Pubkey,  // Mint of the token

    pub title: String,          // title of the proposal: 256
    pub description: String,    // proposal descrition: 1024
    pub proposal_state: State,  // Open, Closed
    pub pass_threhold: u64,     // the min vote to pass the proposal
    pub minimum_token_count_to_vote: u64,
    pub voting_duration: u128,
    
    pub approval_count: u64,
    pub reject_count: u64,
    pub created_timestamp: u128,
    pub opened_timestamp: u128,
    pub closed_timestamp: u128,
}

impl Proposal {
    // Based on account varfiable sizes
    pub const ACCOUNT_SIZE: usize = ACCOUNT_DISCRIMINATOR_LENGTH 
                                    + 4*3 + 256 + 1024 + 1 + 8*4 + 16*4;
}

// Ticket PDA
#[account]
#[derive(Default)] 
pub struct VoteTracker {    
    pub voter: Pubkey,          // voter's token account  
    pub proposal: Pubkey,       // proposal account the voter votes for
    pub vote: VoteOption,
    pub token_amount: u64,      // the amount of token help at the vote.
    pub bump: u8
}

impl VoteTracker {
    // Based on account varfiable sizes
    pub const ACCOUNT_SIZE: usize = ACCOUNT_DISCRIMINATOR_LENGTH 
                                    + 4 + 4 + 1 + 8 + 1;
}
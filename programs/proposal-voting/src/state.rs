use anchor_lang::prelude::*;
//use anchor_spl::token::{TokenAccount, Mint};
use crate::error::ErrorCode;

pub const ACCOUNT_DISCRIMINATOR_LENGTH: usize = 8;

// enums
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initialized,
    Open,
    Closed
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ProposalResult {
    Passed,
    Failed,
    TallyDismatch,
    Cancelled
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VoteOption {
    Approve,
    Reject,
    Abstain
}

impl VoteOption
{
    pub fn new(vote: u8) -> Result<Self> {
        match vote {
            0 => Ok(VoteOption::Approve),
            1 => Ok(VoteOption::Reject),
            2 => Ok(VoteOption::Abstain),
            _ => Err(ErrorCode::InvalidVoteParameter.into()),
        }
    }
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
    pub proposal_result: ProposalResult,  // Open, Closed
    pub pass_threhold: u64,     // the min vote to pass the proposal
    pub minimum_token_count_to_vote: u64,
    
    pub approval_count: u64,
    pub reject_count: u64,
    pub abstain_count: u64,
    pub approval_weighted_count: u64,
    pub reject_weighted_count: u64,
    pub abstain_weighted_count: u64,

    pub tally_approval_count: u64,
    pub tally_reject_count: u64,
    pub tally_abstain_count: u64,

    pub created_timestamp: i64,
    pub vote_end_timestamp: i64,
    pub tally_end_timestamp: i64,
}

impl Proposal {
    // Based on account varfiable sizes
    pub const ACCOUNT_SIZE: usize = ACCOUNT_DISCRIMINATOR_LENGTH 
                                    + 4*3 + 256 + 1024 + 1*2 + 8*12;
}

// Ticket PDA
#[account]
#[derive(Default)] 
pub struct VoteTracker {    
    pub voter_account: Pubkey,          // voter's token account  
    pub proposal: Pubkey,       // proposal account the voter votes for
    pub vote_option: VoteOption,
    pub token_amount: u64,      // the amount of token help at the vote.
}

impl VoteTracker {
    // Based on account varfiable sizes
    pub const ACCOUNT_SIZE: usize = ACCOUNT_DISCRIMINATOR_LENGTH 
                                    + 4 + 4 + 1 + 8 ;
}
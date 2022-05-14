use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

// enums
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initialized,
    Open,
    Closed
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Vote {
    Approve,
    Reject
}

// Accounts
#[account]
pub struct Proposal {    
    pub id: u32,                // unique id for each proposal
    pub authority: Pubkey,      // account creates the proposal
    pub token_account: Pubkey,  // Mint of the token
    pub title: String,          // title of the proposal: 256
    pub description: String,    // proposal descrition: 1024
    pub proposal_state: State,  // Open, Closed
    pub pass_threhold: u64,     // the min vote to pass the proposal
    pub approval_count: u64,
    pub reject_count: u64,
    pub created_timestamp: i64,
    pub opened_timestamp: i64,
    pub closed_timestamp: i64
}

impl Proposal {
    // Based on account varfiable sizes
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 32 + 256 + 1024 + 1 + 64*6;

    // Player that pays for account set up calls this with both pubkeys
    fn new(&mut self, players: [Pubkey; 2]) -> Result<()> {

        Ok(())
    }
}

// Ticket PDA
#[account]
#[derive(Default)] 
pub struct VoteTracker {    
    pub voter: Pubkey,          // voter's token account  
    pub proposal: Pubkey,       // proposal account the voter votes for
    //pub vote: Vote,
    pub token_amount: u64,      // the amount of token help at the vote.
    pub bump: u8
}
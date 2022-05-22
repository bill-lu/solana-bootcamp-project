use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
#[instruction(proposal_id: u32)]
pub struct CreateProposal<'info> {
    #[account(
        init, 
        constraint = token_account.owner == *user.key,
        seeds=[b"proposal_account".as_ref(), token_account.mint.as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump,
        payer = user, 
        space = Proposal::ACCOUNT_SIZE)]
    pub proposal: Account<'info, Proposal>,
    
    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u32)]
pub struct OpenProposal<'info> {
    #[account(
        mut,
        constraint = user.key == &proposal.authority
                    && proposal.proposal_state == State::Initialized,
        seeds=[b"proposal_account".as_ref(), token_account.mint.as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u32)]
pub struct CloseProposal<'info> {
    #[account(
        mut,
        constraint = user.key == &proposal.authority,
        seeds=[b"proposal_account".as_ref(), token_account.mint.as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, vote_option: u8)]
pub struct VoteForProposal<'info> {
    #[account(
        init, 
        constraint = token_account.owner == *user.key 
                && proposal.mint_of_token == token_account.mint 
                && token_account.amount > proposal.minimum_token_count_to_vote
                && proposal.proposal_state == State::Open
                && proposal.vote_end_timestamp > Clock::get().unwrap().unix_timestamp
                && vote_option <= 2,
        seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], 
        bump, 
        payer = user, 
        space = VoteTracker::ACCOUNT_SIZE
    )]
    pub vote_tracker: Account<'info, VoteTracker>,

    #[account(
        mut,
        seeds=[b"proposal_account".as_ref(), token_account.mint.as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
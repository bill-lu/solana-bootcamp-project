use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use crate::state::*;

#[derive(Accounts)]
#[instruction(
    seed: String,
    proposal_id: u32,
    title: String,
    description: String,
    pass_threhold: u64,
    minimum_token_count_to_vote: u64,
    vote_end_timestamp: i64,
    tally_end_timestamp: i64)]
pub struct CreateProposal<'info> {
    #[account(
        init, 
        constraint = token_account.owner == *admin.key,
        seeds=[
            seed.as_bytes(), 
            token_account.mint.as_ref(),
            &proposal_id.to_be_bytes()],
        bump,
        payer = admin, 
        space = Proposal::ACCOUNT_SIZE)]
    pub proposal: Account<'info, Proposal>,
    
    // The token account holding the gated token for this proposal
    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    proposal_seed: String, 
    proposal_id: u32)]
pub struct OpenProposal<'info> {
    #[account(
        mut,
        constraint = user.key == &proposal.authority
                    && proposal.proposal_state == State::Initialized,
        seeds=[
            proposal_seed.as_bytes(), 
            token_account.mint.as_ref(),
            proposal_id.to_be_bytes().as_ref()],
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
#[instruction(
    proposal_seed: String, 
    proposal_id: u32)]
pub struct CloseProposal<'info> {
    #[account(
        mut,
        constraint = user.key == &proposal.authority,
        seeds=[
            proposal_seed.as_bytes(),
            token_account.mint.as_ref(),
            proposal_id.to_be_bytes().as_ref()],
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
#[instruction(
    proposal_seed: String,
    vote_option: u32,
    proposal_id: u32)]
pub struct VoteForProposal<'info> {
    #[account(
        init, 
        constraint = token_account.owner == *user.key 
                && proposal.mint_of_token == token_account.mint 
                && token_account.amount > proposal.minimum_token_count_to_vote
                && proposal.proposal_state == State::Open
                && proposal.vote_end_timestamp > Clock::get().unwrap().unix_timestamp
                && vote_option <= 2,
        seeds = [
            &proposal_id.to_be_bytes(),
            token_account.mint.as_ref(),
            user.key.as_ref()
        ], 
        bump, 
        payer = user, 
        space = VoteTracker::ACCOUNT_SIZE
    )]
    pub vote_tracker: Account<'info, VoteTracker>,

    #[account(
        mut,
        seeds=[
            proposal_seed.as_bytes(), 
            token_account.mint.as_ref(),
            &proposal_id.to_be_bytes()
        ],
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
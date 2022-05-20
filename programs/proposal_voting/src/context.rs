use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
#[instruction(proposal_id: u32)]
pub struct CreateProposal<'info> {
    #[account(
        init, 
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
#[instruction(proposal_id: u64)]
pub struct VoteForProposal<'info> {
    #[account(
        mut,
        seeds=[b"proposal_account".as_ref(), token_account.mint.as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init, 
        seeds = [b"vote_account".as_ref(), proposal_id.to_le_bytes().as_ref(), user.key.as_ref()], 
        bump, 
        payer = user, 
        space = VoteTracker::ACCOUNT_SIZE
    )]
    pub vote_tracker: Account<'info, VoteTracker>,

    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
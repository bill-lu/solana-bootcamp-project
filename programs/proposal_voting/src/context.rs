use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
#[instruction(proposal_id: u32)]
pub struct CreateProposal<'info> {
    #[account(
        init, 
        seeds=[user.key().as_ref(), token_account.key().as_ref(),proposal_id.to_le_bytes().as_ref()],
        bump,
        payer = user, 
        space = Proposal::MAXIMUM_SIZE)]
    pub proposal: Account<'info, Proposal>,
    
    // The token account holding the gated token for this proposal
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
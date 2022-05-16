use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::*;
use crate::context::*;
use crate::utility::Utility;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod error;
pub mod context;
pub mod state;
pub mod utility;

#[program]
pub mod proposal_voting {
    use super::*;

    // TODO: can be configued per community
    pub const MINNIMUM_TOKEN_AMOUNT_TO_CREATE: u64 = 100;

    pub fn initialize_proposal(
        ctx: Context<CreateProposal>, 
        proposal_id: u32,
        title: String,
        description: String,
        pass_threhold: u64,
        minimum_token_count_to_vote: u64,
        voting_duration: u128
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user.to_account_info();
        let current_timestamp = Clock::get()?.unix_timestamp as u128;

        // check if the user holds enough token to create proposal.
        Utility::verify_token_account_amount(
            token_account,
            user.key, 
            MINNIMUM_TOKEN_AMOUNT_TO_CREATE,
        )?; 

        if title.chars().count() > 80 {
            return Err(ErrorCode::TitleIsTooLong.into());
        }

        if description.chars().count() > 1024 {
            return Err(ErrorCode::DescriptionIsTooLong.into());
        }

        proposal.id = proposal_id;
        proposal.authority = *user.key;
        proposal.title = title;
        proposal.description = description;
        proposal.mint_of_token = token_account.mint;

        proposal.proposal_state = State::Initialized;
        proposal.pass_threhold =  pass_threhold;    // the min vote to pass the proposal
        proposal.minimum_token_count_to_vote = minimum_token_count_to_vote;
        proposal.voting_duration = voting_duration;
    
        proposal.approval_count = 0;
        proposal.reject_count = 0;
        proposal.created_timestamp = current_timestamp;
        //proposal.opened_timestamp
        //proposal.closed_timestamp

        Ok(())
    }
}
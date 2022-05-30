use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::*;
use crate::context::*;
use crate::utility::Utility;

declare_id!("84FV6Zmu8WLxmUqY5GFKKkkmbzm9CrpfEkf63qTQtdCE");

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
        _seed: String, 
        proposal_id: u32,
        title: String,
        description: String,
        pass_threhold: u64,
        minimum_token_count_to_vote: u64,
        vote_end_timestamp: i64,
        tally_end_timestamp: i64
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let token_account = &ctx.accounts.token_account;
        let user = &ctx.accounts.admin.to_account_info();

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
        
        proposal.proposal_state = State::Initialized;
        proposal.approval_count = 0;
        proposal.reject_count = 0;
        proposal.tally_approval_count = 0;
        proposal.tally_reject_count = 0;
        proposal.created_timestamp = Clock::get()?.unix_timestamp ;
        proposal.vote_end_timestamp = vote_end_timestamp;
        proposal.tally_end_timestamp = tally_end_timestamp;

        Ok(())
    }

    pub fn open_proposal(
        ctx: Context<OpenProposal>, 
        _proposal_id: u32
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.proposal_state = State::Open;
        Ok(())
    }

    pub fn close_proposal(
        ctx: Context<OpenProposal>, 
        _proposal_id: u32
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.proposal_state = State::Closed;
        Ok(())
    }

    pub fn vote(
        ctx: Context<VoteForProposal>, 
        vote_option: u8,
        _proposal_id: u32
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let token_account = &mut ctx.accounts.token_account;
        let user = &mut ctx.accounts.user;
        let voter_tracking = &mut ctx.accounts.vote_tracker;

        proposal.proposal_state = State::Closed;

        voter_tracking.voter_account = user.key();
        voter_tracking.proposal = proposal.key();
        voter_tracking.token_amount = token_account.amount;
        voter_tracking.vote_option = VoteOption::new(vote_option).unwrap();

        match voter_tracking.vote_option {
            VoteOption::Approve => {
                proposal.approval_count += 1;
                proposal.approval_weighted_count += 1*token_account.amount
            },
            VoteOption::Reject => {
                proposal.reject_count += 1;
                proposal.reject_weighted_count += 1*token_account.amount
            },
            VoteOption::Abstain => {
                proposal.abstain_count +=1;
                proposal.abstain_weighted_count += 1* token_account.amount
            }
        }

        Ok(())
    }
}
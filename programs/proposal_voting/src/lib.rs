use anchor_lang::prelude::*;
use crate::state::*;
use crate::context::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod context;
pub mod state;

#[program]
pub mod proposal_voting {
    use super::*;

    pub fn initialize_proposal(ctx: Context<CreateProposal>) -> Result<()> {
        Ok(())
    }
}
use anchor_lang::prelude::*;

// https://project-serum.github.io/anchor/tutorials/tutorial-4.html
#[error_code]
pub enum ErrorCode  {
	#[msg("The requested proposal id does not match the proposal account")]
	ProposalIDMismatch,
	#[msg("You have already voted for this proposal")]
	YouAlreadyVotedForThisProposal,
	#[msg("Title is too long. maximum: 80 character")]
	TitleIsTooLong,
	#[msg("Description is too long. maximum: 1024 character")]
	DescriptionIsTooLong,
	#[msg("Proposal deadline is past")]
	ProposalHasEnded,
	#[msg("Proposal voting is finalized no changes can be made")]
	ProposalVotingFinalized,
	#[msg("The voting time has not ended")]
	VotingTimeHasNotEnded,
	#[msg("The time for tallying votes has not ended wait until tally time has ended")]
	FinalizeVoteTimeHasNotEnded,
	#[msg("The timestamp of voting end must be greater than the current timestamp")]
	VotingEndTimestampTooSmall,
	#[msg("The timestamp of finalizing proposal must be greater than the voting end timestamp and current timestamp")]
	FinalizeTimestampTooSmall,
	#[msg("Insufficient number of tokens to be included in voting session")]
	InsufficientTokensToVote,
	#[msg("The vote parameter entered is invalid for this type of voting")]
	InvalidVoteParameter,
	#[msg("The token account is not the correct mint")]
	InvalidTokenAccount,
	#[msg("The token account balance is less than the required balance for the community")]
	InsufficientTokenBalance,
	#[msg("The user is not the owner of the token account")]
	UnauthorizedTokenHolder,
	#[msg("The account provided is not owned by the Qwestive Voting program")]
	UnauthorizedAccount,
	#[msg("The metadata account provided does not match the expected key")]
	MetadataAccountMismatch,
}
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AccountInfo, clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
//import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount } from "@solana/spl-token";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { assert, expect } from "chai";
import * as bs58 from "bs58";

import { ProposalVoting } from "../target/types/proposal_voting";

const { SystemProgram } = anchor.web3;

describe("proposal-voting", () => {
  // Configure the client to use the local cluster.
  //const provider = anchor.AnchorProvider.env();
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.ProposalVoting as Program<ProposalVoting>;

  // accounts etc
  //let mint1, mint2; // spl-token 0.2.0
  let mint1: Token = null;
  let mint2: Token = null;

  const mintAuthority1 = anchor.web3.Keypair.generate();
  const mintAuthority2 = anchor.web3.Keypair.generate();

  let voter1WithToken1Pubkey, voter2WithToken1Pubkey, voter3WithToken1Pubkey, voter4WithToken1Pubkey;
  let voterWithToken2Pubkey;

  // token amount for accounts
  const voter1Token1Balance = 2000
  const voter2Token1Balance = 1000
  const voter3Token1Balance = 750
  const voter4Token1Balance = 5
  const voterToken2Balance = 5000

  // wallets accounts
  const payer1 = anchor.web3.Keypair.generate();
  const payer2 = anchor.web3.Keypair.generate();
  const payer3 = anchor.web3.Keypair.generate();
  const payer4 = anchor.web3.Keypair.generate();

  const UNIX_MS_FACTOR = 1000;
  const DAY_IN_UNIX = 24 * 60 * 60;
  const HOURS_IN_UNIX = 60 * 60;
  const MINUTES_IN_UNIX = 60;
  const SECONDS_IN_UNIX = 1;

  // Global addresses for easy loading to subsequent tests
  let bump;
  let proposalPDA;
  let adminTokenAccount;

  before(async () => {
    await initializeMintsAndAccounts();
  });

  it("User without enough token should not be able to create a proposal", async () => {
    // Seed for proposalPDA
    let seedString: string = "proposal_account";
    let seed: Buffer = Buffer.from(seedString);

    const proposalID: number = 1;
    let title: string = `first proposal`;
    let desc: string = `some proposal`;
    const proposalIdBuffer = getNumberBuffer(proposalID);

    [proposalPDA, bump] = await anchor.web3.PublicKey.findProgramAddress(
                        [seed, 
                          Buffer.from(mint1.publicKey.toBytes()),
                          proposalIdBuffer
                        ],
                        program.programId
      );

    try {
      await program.methods.initializeProposal(
                  seedString,            
                  proposalID,
                  title, 
                  desc, 
                  new anchor.BN(100), 
                  new anchor.BN(900),
                  new anchor.BN((+new Date() / UNIX_MS_FACTOR) + 1 * DAY_IN_UNIX), //voting_end_timestamp
                  new anchor.BN((+new Date() / UNIX_MS_FACTOR) + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
                  )
                .accounts({
                  proposal: proposalPDA,
                  tokenAccount: voter4WithToken1Pubkey,
                  admin: payer4.publicKey,
                  systemProgram: SystemProgram.programId,
                })
                .signers([payer4])
                .rpc()
      assert(false);
    } catch (err) {
      assert(err);
    }
  });

  it("User with enough token should be able to create a proposal", async () => {
    // Seed for proposalPDA
    let seedString: string = "proposal_account";
    let seed: Buffer = Buffer.from(seedString);

    const proposalID: number = 1;
    let title: string = `first proposal`;
    let desc: string = `some proposal`;
    const proposalIdBuffer = getNumberBuffer(proposalID);

    [proposalPDA, bump] = await anchor.web3.PublicKey.findProgramAddress(
                        [seed, 
                          Buffer.from(mint1.publicKey.toBytes()),
                          proposalIdBuffer
                        ],
                        program.programId
      );

    await program.methods.initializeProposal(
                  seedString,            
                  proposalID,
                  title, 
                  desc, 
                  new anchor.BN(100), 
                  new anchor.BN(900),
                  new anchor.BN((+new Date() / UNIX_MS_FACTOR) + 1 * DAY_IN_UNIX), //voting_end_timestamp
                  new anchor.BN((+new Date() / UNIX_MS_FACTOR) + 2 * DAY_IN_UNIX), //finalize_vote_end_timestamp
                  )
                .accounts({
                  proposal: proposalPDA,
                  tokenAccount: voter1WithToken1Pubkey,
                  admin: payer1.publicKey,
                  systemProgram: SystemProgram.programId,
                })
                .signers([payer1])
                .rpc()
  });










  // Utilities
  const getNumberBuffer = (total: number, alloc = 4) => {
    const totalProposalAccountBuf = Buffer.alloc(alloc);
    totalProposalAccountBuf.writeUIntBE(total, 0, 4);
    return totalProposalAccountBuf;
  };

  async function initializeMintsAndAccounts() {
    
    // airdrop SOL
    console.log("Airdrop");
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer1.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer2.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer3.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer4.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    console.log("Create mints");
    mint1 = await Token.createMint (
      provider.connection,
      payer2,
      mintAuthority1.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    mint2 = await Token.createMint (
      provider.connection,
      payer3,
      mintAuthority2.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    console.log("create token association accounts");
    // mint1 associated token accounts
    adminTokenAccount = await mint1.getOrCreateAssociatedAccountInfo (
      payer1.publicKey
    );
    voter1WithToken1Pubkey = adminTokenAccount.address;
/*     voter1WithToken1Pubkey = (await mint1.getOrCreateAssociatedAccountInfo (
      payer1.publicKey
    )).address; */
    console.log("voter1WithToken1Pubkey: " + voter1WithToken1Pubkey);
    
    voter2WithToken1Pubkey = (await mint1.getOrCreateAssociatedAccountInfo (
      payer2.publicKey
    )).address;

    voter3WithToken1Pubkey = (await mint1.getOrCreateAssociatedAccountInfo (
      payer3.publicKey
    )).address;

    voter4WithToken1Pubkey = (await mint1.getOrCreateAssociatedAccountInfo (
      payer4.publicKey
    )).address;

    // Mint2 associated account
    voterWithToken2Pubkey = (await mint2.getOrCreateAssociatedAccountInfo (
      payer2.publicKey
    )).address;

    console.log("minting tokens");
    // mint tokens to accounts.

    console.log("voter1WithToken1Pubkey: " + voter1WithToken1Pubkey);
    console.log("mintAuthority1.publicKey: " + mintAuthority1.publicKey);
    console.log("voter1Token1Balance: " + voter1Token1Balance);
    console.log("mint1: " + mint1);
    console.log("payer2: " + payer2.publicKey);

    let result1 = await mint1.mintTo(
      voter1WithToken1Pubkey,
      mintAuthority1.publicKey,
      [mintAuthority1],
      voter1Token1Balance
    );

    await mint1.mintTo(
      voter2WithToken1Pubkey,
      mintAuthority1.publicKey,
      [mintAuthority1],
      voter2Token1Balance
    );

    console.log("minting tokens: " + result1);

    await mint1.mintTo(
      voter3WithToken1Pubkey,
      mintAuthority1.publicKey,
      [mintAuthority1],
      voter3Token1Balance
    );

    await mint1.mintTo(
      voter4WithToken1Pubkey,
      mintAuthority1.publicKey,
      [mintAuthority1],
      voter4Token1Balance
    );

    await mint2.mintTo(
      voterWithToken2Pubkey,
      mintAuthority2.publicKey,
      [mintAuthority2],
      voterToken2Balance
    );
    

    console.log("checking ...");
    let _voter1TAA = await mint1.getAccountInfo(voter1WithToken1Pubkey);
    console.log("Token Balance ..." + _voter1TAA.amount.toString());
    expect(_voter1TAA.amount.toString()).to.equal((voter1Token1Balance).toString());

  }

  /*
  // with spl-token 0.2.0
  async function initializeMintsAndAccounts() {
    
    // airdrop SOL
    console.log("Airdrop");
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer1.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer2.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer3.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer4.publicKey, 2 * LAMPORTS_PER_SOL),
      "confirmed"
    );

    console.log("Create mints");
    mint1 = await createMint (
      provider.connection,
      payer2,
      mintAuthority1.publicKey,
      null,
      9
    );

    mint2 = await createMint (
      provider.connection,
      payer3,
      mintAuthority2.publicKey,
      null,
      9
    );

    console.log("create token association accounts");
    // mint1 associated token accounts
    voter1WithToken1Pubkey = (await getOrCreateAssociatedTokenAccount (
      provider.connection,
      payer1,
      mint1,
      payer1.publicKey
    )).address;

    console.log("voter1WithToken1Pubkey: " + voter1WithToken1Pubkey);
    voter2WithToken1Pubkey = (await getOrCreateAssociatedTokenAccount (
      provider.connection,
      payer2,
      mint1,
      payer2.publicKey
    )).address;

    voter3WithToken1Pubkey = (await getOrCreateAssociatedTokenAccount (
      provider.connection,
      payer3,
      mint1,
      payer3.publicKey
    )).address;

    voter4WithToken1Pubkey = (await getOrCreateAssociatedTokenAccount (
      provider.connection,
      payer4,
      mint1,
      payer4.publicKey
    )).address;

    // Mint2 associated account
    voterWithToken2Pubkey = (await getOrCreateAssociatedTokenAccount (
      provider.connection,
      payer2,
      mint2,
      payer2.publicKey
    )).address;

    console.log("minting tokens");
    // mint tokens to accounts.

    console.log("voter1WithToken1Pubkey: " + voter1WithToken1Pubkey);
    console.log("mintAuthority1.publicKey: " + mintAuthority1.publicKey);
    console.log("voter1Token1Balance: " + voter1Token1Balance);
    console.log("mint1: " + mint1);
    console.log("payer2: " + payer2.publicKey);

    let result1 = await mintTo(
      provider.connection,
      payer1,
      mint1,
      voter1WithToken1Pubkey,
      mintAuthority1.publicKey,
      voter1Token1Balance
    );

    await mintTo(
      provider.connection,
      payer2,
      mint1,
      voter2WithToken1Pubkey,
      mintAuthority1.publicKey,
      voter2Token1Balance * 10e9
    );

    console.log("minting tokens: " + result1);

    await mintTo(
      provider.connection,
      payer3,
      mint1,
      voter3WithToken1Pubkey,
      mintAuthority1.publicKey,
      voter3Token1Balance
    );

    await mintTo(
      provider.connection,
      payer4,
      mint1,
      voter4WithToken1Pubkey,
      mintAuthority1.publicKey,
      voter4Token1Balance
    );

    await mintTo(
      provider.connection,
      payer2,
      mint2,
      voterWithToken2Pubkey,
      mintAuthority2.publicKey,
      voterToken2Balance
    );
    

    console.log("checking ...");
    let _voter1TAA = await getAccount(provider.connection, voter1WithToken1Pubkey);
    expect(_voter1TAA.amount.toString()).to.equal((voter1Token1Balance + 1).toString());

  }
  */
});

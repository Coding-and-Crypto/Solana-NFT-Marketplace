import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import { NftMarketplace } from "../target/types/nft_marketplace";

describe("nft-marketplace", async () => {
  
  const provider = anchor.AnchorProvider.env()
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  // ** Un-comment this to use solpg imported IDL **
  // const program = new anchor.Program(
  //   require("../solpg/idl.json"), 
  //   new anchor.web3.PublicKey("H2UJjAQTuVJYhaBhh6GD2KaprLBTp1vhP2aaHioya5NM"),
  // );
  // ** Comment this to use solpg imported IDL **
  const program = anchor.workspace.NftMarketplace as anchor.Program<NftMarketplace>;
  
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const tokenAddress = await anchor.utils.token.associatedAddress({
    mint: mintKeypair.publicKey,
    owner: wallet.publicKey
  });

  const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  ))[0];
  const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID
  ))[0];

  it("Mint!", async () => {
    
    const tx = await program.methods.mint(
      "Test NFT", "TEST", "https://raw.githubusercontent.com/Coding-and-Crypto/Rust-Solana-Tutorial/master/nft-marketplace/nft-example.json"
    )
    .accounts({
      masterEdition: masterEditionAddress,
      metadata: metadataAddress,
      mint: mintKeypair.publicKey,
      mintAuthority: mintKeypair.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenAccount: tokenAddress,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([mintKeypair])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});

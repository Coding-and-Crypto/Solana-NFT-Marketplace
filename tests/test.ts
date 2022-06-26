import * as anchor from "@project-serum/anchor";
import {
    createKeypairFromFile,
  } from './util';
// ** Comment this to use solpg imported IDL **
import { NftMarketplace } from "../target/types/nft_marketplace";


describe("nft-collection", async () => {
  
  const testNftTitle = "Beta";
  const testNftSymbol = "BETA";
  const testNftUri = "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json";

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

  // Derive the mint address and the associated token account address

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  console.log(`\n\nOur new NFT token will be: ${mintKeypair.publicKey}`);


  it("Mint!", async () => {

    console.log("\n\n");
    console.log("Processing mint...");

    // Derive the associated token account address

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey
    });
    console.log(`Your (minter) token address: ${mintKeypair.publicKey}`);

    // Derive the metadata and master edition addresses

    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];
    console.log(`Your NFT's metadata address: ${metadataAddress}`);
    const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];
    console.log(`Your NFT's master edition metadata address: ${masterEditionAddress}`);

    // Transact with the "mint" function in our on-chain program
    
    await program.methods.mint(
      testNftTitle, testNftSymbol, testNftUri
    )
    .accounts({
      masterEdition: masterEditionAddress,
      metadata: metadataAddress,
      mint: mintKeypair.publicKey,
      tokenAccount: tokenAddress,
      mintAuthority: wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([mintKeypair])
    .rpc();

    console.log("Transaction successful.");
  });

//   it("Bid!", async () => {});

  it("Sell!", async () => {

    console.log("\n\n");

    // Testing constants

    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const buyer: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer1.json");
    console.log(`Selling NFT: ${mintKeypair.publicKey}`);
    console.log(`Owner's public key: ${wallet.publicKey}`);
    console.log(`Buyer's public key: ${buyer.publicKey}`);
    console.log(`The buyer has requested to purchase NFT: ${mintKeypair.publicKey} for ${saleAmount} lamports.`);
    console.log("Processing sale...");

    // Derive the associated token account address for owner & buyer

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey
    });
    const buyerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: buyer.publicKey,
    });
    console.log(`Owner's Token Address: ${ownerTokenAddress}`);
    console.log(`Buyer's Token Address: ${buyerTokenAddress}`);

    // Transact with the "sell" function in our on-chain program
    
    await program.methods.sell(
      new anchor.BN(saleAmount)
    )
    .accounts({
      mint: mintKeypair.publicKey,
      ownerTokenAccount: ownerTokenAddress,
      ownerAuthority: wallet.publicKey,
      buyerTokenAccount: buyerTokenAddress,
      buyerAuthority: buyer.publicKey,
    })
    .signers([buyer])
    .rpc();

    console.log("Transaction successful.");
  });
});

import * as anchor from "@project-serum/anchor";
// ** Comment this to use solpg imported IDL **
import {
  createKeypairFromFile,
} from './util';
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


  it("Sell!", async () => {

    // Testing constants

    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "HQjS4Yp1Ku7P1fy4YwwPACeaDLY9AC62b3omfBSQ5jyT"
    );
    // const buyerPubkey: anchor.web3.PublicKey = new anchor.web3.PublicKey(
    //   "3ec3aeqsDN6yyNqgvi2HxJ8AhY3nwhQBB6L6LPsHAeqX"
    // );
    // const buyer: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const buyer: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer1.json");
    console.log(`Buyer public key: ${buyer.publicKey}`);

    // Derive the associated token account address for owner & buyer

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: wallet.publicKey
    });
    const buyerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: buyer.publicKey,
    });
    console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
    console.log(`Owner's Token Address: ${ownerTokenAddress}`);
    console.log(`Buyer's Token Address: ${buyerTokenAddress}`);

    // Transact with the "sell" function in our on-chain program
    
    await program.methods.sell(
      new anchor.BN(saleAmount)
    )
    .accounts({
      mint: mint,
      ownerTokenAccount: ownerTokenAddress,
      ownerAuthority: wallet.publicKey,
      buyerTokenAccount: buyerTokenAddress,
      buyerAuthority: buyer.publicKey,
    })
    .signers([buyer])
    .rpc();
  });
});

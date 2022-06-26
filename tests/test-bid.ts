import * as anchor from "@project-serum/anchor";
import {
  createKeypairFromFile,
} from './util';
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

    it("Bid!", async () => {

    // Testing constants

    const bidAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "12zajHBLHcPFmvRMiBtgkBu3bjvjvcFf7eq2SACWGqbG"
    );
    console.log(`Mint: ${mint}`);

    const buyer: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer1.json");
    console.log(`Buyer public key: ${buyer.publicKey}`);

    const bidAccountAddress = await anchor.web3.PublicKey.createProgramAddress(
      [Buffer.from("bid_"), Buffer.from(mint.toString())],
      program.programId,
    );
    console.log(`Buyer's Bid Account key: ${bidAccountAddress}`);

    // Transact with the "pda" function in our on-chain program
    
    await program.methods.bid(
      new anchor.BN(bidAmount)
    )
    .accounts({
      mint: mint,
      bidAccount: bidAccountAddress,
      authority: buyer.publicKey,
    })
    .signers([buyer])
    .rpc();

  });
});

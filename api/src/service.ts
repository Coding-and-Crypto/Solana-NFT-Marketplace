import * as anchor from "@project-serum/anchor";


export type Nft = JSON;


// Mint a new random NFT from our collection.
//
export async function mintNFt(): Promise<Nft[]> {
    let x: Nft[] = [];
    return x;
}

// Look up any NFTs that have been minted by our collection.
//
export async function getNftsMinted(): Promise<Nft[]> {
    let x: Nft[] = [];
    return x;
}

// Place a bid on an NFT.
//
export async function bidOnNft(
    mintAddress: anchor.web3.PublicKey,
    lamports: number,
): Promise<Nft[]> {

    let x: Nft[] = [];
    return x;
}

// Get all NFTs owned by your wallet.
//
export async function getNftsOwned(
    publicKey: anchor.web3.PublicKey
): Promise<Nft[]> {
    
    let x: Nft[] = [];
    return x;
}

// Sell an NFT to the highest bidder.
//
export async function sellNft(
    mintAddress: anchor.web3.PublicKey,
    ownerPublicKey: anchor.web3.PublicKey,
): Promise<Nft[]> {
    
    let x: Nft[] = [];
    return x;
}


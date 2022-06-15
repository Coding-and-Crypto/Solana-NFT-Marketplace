import * as anchor from "@project-serum/anchor";
import { Request, Response, Router } from 'express';
import axios, { AxiosResponse } from 'axios'; // Needed for client calls
import * as service from "./service";

const router = Router();

// Mint a new random NFT from our collection.
//
router.get('/mint-new', async (req: Request, res: Response) => {
    return res.status(200).json(
        await service.mintNFt()
    );
});

// Look up any NFTs that have been minted by our collection.
//
router.get('/get-nfts-minted', async (req: Request, res: Response) => {
    return res.status(200).json(
        await service.getNftsMinted()
    );
});

// Place a bid on an NFT.
//
router.post('/bid-on-nft', async (req: Request, res: Response) => {
    return res.status(200).json(
        await service.bidOnNft(
            req.body.mintAddress,
            req.body.lamports,
        )
    );
});

// Get all NFTs owned by your wallet.
//
router.get('/get-nfts-owned/:publicKey', async (req: Request, res: Response) => {
    return res.status(200).json(
        await service.getNftsOwned(
            new anchor.web3.PublicKey(req.params.publicKey)
        )
    );
});

// Sell an NFT to the highest bidder.
//
router.post('/sell-nft', async (req: Request, res: Response) => {
    return res.status(200).json(
        await service.sellNft(
            req.body.mintAddress,
            req.body.ownerPublicKey,
        )
    );
});

export = router;
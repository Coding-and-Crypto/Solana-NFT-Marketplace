import { Request, Response, Router } from 'express';
import axios, { AxiosResponse } from 'axios'; // Needed for client calls
import { getNftsMinted } from "./service";

const router = Router();

/// Look up any NFTs that have been minted by our collection.

router.get('/nfts-minted', async (req: Request, res: Response) => {
    let nftsMinted: string[] = [];
    return res.status(200).json({nfts : await getNftsMinted()});
});


export = router;
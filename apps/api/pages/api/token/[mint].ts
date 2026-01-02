import type { NextApiRequest, NextApiResponse } from 'next'
import { TokenLaunch } from '@legal/shared'

// NOTE: This endpoint is a minimal verifier. In production it should query Solana RPC
export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const { mint } = req.query
  if (!mint || typeof mint !== 'string') return res.status(400).json({ error: 'mint required' })

  // Minimal mocked response to demonstrate shape
  const token: TokenLaunch = {
    mint,
    creator: 'ExampleCreatorPubkey1111111111111111111111111',
    supply: '1000000000',
    decimals: 6,
    launchedAt: Math.floor(Date.now() / 1000),
  }

  res.status(200).json({ ...token, safe: true, mintAuthority: null })
}

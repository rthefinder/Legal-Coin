## How to verify the contract and a token launch

1. Build the Anchor IDL: `anchor build` will write `target/idl/legal.json`.
2. Run the static checks: `pnpm --filter @legal/security run build && pnpm --filter @legal/security run check-idl target/idl/legal.json`
3. Launch on localnet and inspect the transaction logs for `Launch` and `AuthorityRevoked` events.
4. Inspect the mint account using `solana account <MINT_PUBKEY> --output json` and verify `mintAuthority` is null and `supply` matches launched amount.

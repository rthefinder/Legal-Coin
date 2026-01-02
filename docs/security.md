**Threat Model & Security Notes**

This document outlines the basic threat model for $LEGAL and describes what the system guarantees.

Assumptions
- Users interact with the system via a client using a wallet (Phantom, Solflare).
- The Solana token program is trusted to enforce token semantics.

Guarantees
- Supply immutability: All minted supply is created during the launch transaction.
- Authorities revoked: Mint and freeze authorities are set to None immediately.
- No admin backdoors: The Anchor contract includes no functions to re-set authorities.

Adversary model
- A malicious creator cannot mint additional tokens after launch because authorities are revoked.
- A malicious creator cannot withdraw LP if the liquidity flow is enforced externally and verified.

Verification steps
1. Retrieve the mint account and confirm `supply` equals launched supply.
2. Confirm `mint_authority` === null and `freeze_authority` === null.
3. Confirm program events for `Launch` and `AuthorityRevoked` in the transaction logs.

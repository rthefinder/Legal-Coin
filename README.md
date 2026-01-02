# $LEGAL — Legal Coin

<img width="1500" height="500" alt="legalbanner" src="https://github.com/user-attachments/assets/b8bf1dc0-0435-41ab-bc64-93b777872db9" />

**What is $LEGAL**

$LEGAL is a security-first memecoin launchpad and verification standard built for Pump.fun-compatible token launches on Solana. It provides a strict, auditable launch flow that prevents rugpulls, bundles, dev-snipes, hidden mints, and supply manipulation.

This repository contains a full-stack, open-source reference implementation: an Anchor smart contract, a Next.js frontend and API, static verifiers, tests, CI, and developer tooling.

Key principles:
- Security over speed
- Transparency over flexibility
- No trust assumptions
- Everything verifiable on-chain or via open-source code

What $LEGAL is not:
- Not financial advice
- Not an investment platform
- Not a custody or managed-liquidity solution

<img width="602" height="997" alt="Capture d’écran 2026-01-02 à 14 59 29" src="https://github.com/user-attachments/assets/781f28fd-8fb4-461e-a5e5-77f388f71940" />

Directory layout

- `/apps/web` — Next.js frontend (TypeScript, Tailwind)
- `/apps/api` — Next.js API routes for public read endpoints
- `/programs/legal` — Anchor (Rust) smart contract enforcing safe launch flow
- `/packages/shared` — Shared TypeScript types/constants
- `/packages/security` — Static checks & validators
- `/tests` — Integration tests using Anchor tools
- `/infra` — Local dev tooling (Docker files)
- `/docs` — Architecture and security docs
- `/.github/workflows` — CI pipelines

<img width="917" height="845" alt="Capture d’écran 2026-01-02 à 14 56 12" src="https://github.com/user-attachments/assets/018866bd-37a7-46a6-bdd2-fd5a817d45f1" />

Quick commands

Install:
```
pnpm install
```

Run local dev (frontend + api):
```
pnpm dev
```

Contract tests (local validator):
```
cd programs/legal
anchor test
```

Typecheck + lint:
```
pnpm -w run typecheck
pnpm -w run lint
```

Deploy contracts (local):
```
anchor build
anchor deploy --provider.cluster localnet
```

Security guarantees (plain English)
- Token supply is fixed at launch and minted in a single transaction.
- Mint and freeze authorities are revoked immediately and irreversibly.
- The launch is recorded on-chain by a simple, auditable program that emits events for launch, authority revocation, and trading enablement.
- No admin backdoors or pause switches are present in the contract.

How to verify a launch
1. Inspect the token mint: total supply equals launched supply.
2. Authorities: mint authority === none, freeze authority === none.
3. Program events: `Launch`, `AuthorityRevoked`, `TradingEnabled` appear in the minting transaction logs.
4. Optionally run `/packages/security` checks against the program IDL and token mint state.

Disclaimer

This software is experimental and not financial advice. Use at your own risk.

Next steps
- Run `pnpm install`, then `pnpm dev`.
- Review `/docs/security.md` and the Anchor contract in `/programs/legal/src/lib.rs`.
# Legal-Coin
Fraud has made a name for itself in almost every realm of crypto - Degecoin rugging, Rizzmass rugging,  so why not flip this narrative?

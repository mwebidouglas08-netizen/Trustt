# Architecture (TRUSTLAYER)

- **Web:** Next.js (App Router) in `apps/web`, deployed to Vercel.
- **Backend data:** Supabase (Postgres, Auth, Storage, Edge Functions). RLS required on exposed tables.
- **Chain:** Solana Anchor program `trustlayer_escrow` (USDC vault escrow). Client verification via RPC; server functions verify transactions before mirroring state.
- **Non-custodial:** Users sign deposits/releases/disputes in their wallets; program holds vault under PDA authority.

See root `TRUSTLAYER_TryNoah_Fullstack_Build_Specification.md` in the Downloads folder reference context was provided; keep this repo’s docs in sync with implemented behavior.

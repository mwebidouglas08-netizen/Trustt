# TRUSTLAYER

Trade with confidence. Monorepo for the TRUSTLAYER transaction protection app (Next.js web, Supabase, Solana Anchor escrow).

## Status
Scaffold + core specs implemented in-repo. **Not** production-deployed. Devnet program ID in config is a placeholder until you deploy `trustlayer_escrow`.

## Layout
- `apps/web` — Next.js App Router frontend
- `programs/trustlayer_escrow` — Anchor program (USDC escrow)
- `supabase/migrations`, `supabase/functions`, `supabase/seed.sql`
- `docs/` — architecture, deployment, threat model, etc.
- `.env.example` — env template (no secrets)

## Quick start (web)
```bash
pnpm install
cp .env.example apps/web/.env.local
# fill Supabase + public Solana vars
pnpm -C apps/web dev
```

## Program
```bash
cd programs/trustlayer_escrow
anchor build
anchor test
```
Update `Anchor.toml` and program ID after generating/deploying a keypair. Do not use placeholder program IDs on mainnet.

## Documentation
- Full build spec (source of truth provided): see project handoff document.
- `docs/deployment.md`, `docs/architecture.md`, `docs/threat-model.md`

## Contributing
Follow the non-negotiable rules in the build spec: no fake on-chain states, no secrets in client, server-verified chain writes only.

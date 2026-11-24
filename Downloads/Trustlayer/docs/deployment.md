# Deployment

## Prerequisites
- Node.js 20+, pnpm 9+
- Supabase CLI (optional for local), Supabase project
- Solana CLI + Anchor 0.30.x for program work

## Web (Vercel)
1. Import repo, set root to repo root; configure build for `apps/web` (`pnpm install && pnpm -C apps/web build`) or use Vercel settings.
2. Set env vars from `.env.example` (public vars only in browser).
3. Separate preview vs production envs; never point production escrow to test data unintentionally.

## Supabase
- Apply migrations in `supabase/migrations` in order.
- Deploy Edge Functions from `supabase/functions` (create functions as specified in main spec).
- Configure Auth (email confirm), Storage buckets (private), RLS.

## Solana
- Devnet first. Build: `cd programs/trustlayer_escrow && anchor build`. Deploy with secured wallet; record program ID and update Anchor.toml + env.
- Mainnet only after audit, multisig, legal review.

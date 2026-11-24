# Threat model (draft)

Assets: escrowed USDC, user accounts, private evidence, dispute records, program authority keys.

Trust boundaries: browser ↔ Next.js ↔ Supabase ↔ Solana RPC ↔ on-chain program. Server (Edge Functions/service role) is privileged for verification and writes; never expose service role to browser.

Key risks: phishing/wallet loss, RPC manipulation (mitigate by verifying tx signatures on server), malicious uploads (validate + scan), RLS misconfiguration, compromised arbitration keys (use multisig in production).

Residual risks: physical delivery fraud, legal enforceability varies by jurisdiction, AI summaries can be wrong (must be labelled, human decides).

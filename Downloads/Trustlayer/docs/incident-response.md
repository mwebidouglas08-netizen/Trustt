# Incident response (draft)

- **Pause:** Use on-chain pause (when implemented) to stop new escrows if needed; existing funded escrows require careful handling (do not trap funds).
- **Key compromise:** Rotate authorities via secure multisig procedures; revoke API keys; rotate Supabase/service credentials.
- **Reconciliation mismatch:** Freeze affected flows in UI (read-only), investigate chain vs DB, do not manually “fix” funds off-chain.
- **Comms:** Notify users via in-app/email using approved templates; do not leak secrets.

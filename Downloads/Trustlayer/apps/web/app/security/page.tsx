export default function SecurityPage() {
  return (
    <div className="trust-container py-12 max-w-3xl">
      <h1 className="text-3xl font-semibold text-ink-900">Security</h1>
      <div className="mt-6 space-y-4 text-slate-700 text-sm leading-relaxed">
        <p><strong>Custody:</strong> Non-custodial. You sign with your own wallet. Funds move only through on-chain program instructions you authorize (deposit, release, or authorized dispute resolution).</p>
        <p><strong>Data:</strong> Private evidence and personal details are stored off-chain in Supabase with access controls and signed URLs—not in public Solana account data.</p>
        <p><strong>Transparency:</strong> Program addresses, mints, and transactions are verifiable on-chain. This site does not claim audits unless a real report is published.</p>
        <p><strong>Limits:</strong> No system can guarantee delivery of physical goods by blockchain alone. Disputes require human review under policy.</p>
      </div>
    </div>
  );
}

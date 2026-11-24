export default function FeesPage() {
  return (
    <div className="trust-container py-12 max-w-3xl">
      <h1 className="text-3xl font-semibold text-ink-900">Fees</h1>
      <p className="mt-4 text-slate-700">MVP default platform fee is <strong>0%</strong> unless configured otherwise by operators with legal review.</p>
      <ul className="mt-6 list-disc list-inside space-y-2 text-slate-700 text-sm">
        <li>You pay Solana network fees (SOL) required by the blockchain for transactions you sign.</li>
        <li>Any platform fee, if enabled later, will be shown before you fund an escrow and is capped by program rules.</li>
        <li>Fees are never hidden. Exact amounts appear in the transaction review before signing.</li>
      </ul>
    </div>
  );
}

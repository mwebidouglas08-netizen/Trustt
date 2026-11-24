export default function HowItWorksPage() {
  return (
    <div className="trust-container py-12 max-w-3xl">
      <h1 className="text-3xl font-semibold text-ink-900">How TRUSTLAYER works</h1>
      <p className="mt-4 text-slate-600">TRUSTLAYER is a transaction-protection layer, not a marketplace.</p>
      <ol className="mt-8 space-y-6 list-decimal list-inside text-slate-700">
        <li><strong>Agree:</strong> buyer and seller confirm terms, amount (USDC), deadlines, and acceptance criteria. Both accept the same agreement version.</li>
        <li><strong>Secure:</strong> buyer funds a non-custodial Solana escrow vault controlled by the on-chain program. Seller cannot withdraw funds directly.</li>
        <li><strong>Complete:</strong> buyer reviews evidence and releases funds on-chain, or opens a dispute under the published policy for human review and authorized settlement.</li>
      </ol>
      <p className="mt-8 text-sm text-slate-600">Blockchain records the transaction; it does not prove a physical item was delivered. Evidence and disputes remain off-chain and policy-driven.</p>
    </div>
  );
}

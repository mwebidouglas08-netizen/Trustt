export default function DisputesPage() {
  return (
    <div className="trust-container py-12 max-w-3xl">
      <h1 className="text-3xl font-semibold text-ink-900">Dispute policy (draft)</h1>
      <p className="mt-4 text-slate-700 text-sm">This is a draft summary for development. Qualified legal counsel must review before production.</p>
      <div className="mt-6 space-y-4 text-slate-700 text-sm">
        <p>Disputes are reviewed by an authorized arbitrator under a published policy. AI may summarize evidence but does not make final decisions or move funds.</p>
        <p>Possible outcomes depend on program support and policy: full refund to buyer, full payment to seller, or (if implemented and tested) a documented split.</p>
        <p>Outcomes are executed via authorized on-chain instructions; admins cannot transfer escrow funds by editing the database.</p>
      </div>
    </div>
  );
}

export default function NewTransactionPage() {
  return (
    <div className="trust-container py-10 max-w-2xl">
      <h1 className="text-2xl font-semibold text-ink-900">Create protected transaction</h1>
      <p className="mt-2 text-sm text-slate-600">Draft form UI placeholder. Full validation, invitation flow, and on-chain steps require backend wiring (Supabase + Anchor).</p>
      <form className="mt-8 space-y-6" action="#">
        <div>
          <label className="block text-sm font-medium text-ink-800" htmlFor="title">Title</label>
          <input id="title" className="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm" placeholder="e.g. MacBook Pro sale" disabled />
        </div>
        <div>
          <label className="block text-sm font-medium text-ink-800" htmlFor="amount">Amount (USDC)</label>
          <input id="amount" className="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm" placeholder="0.00" disabled />
        </div>
        <p className="text-xs text-slate-500">Implementation note: enforce server-side validation, agreement hashing, and do not store sensitive PII in chain data.</p>
        <button type="button" disabled className="rounded-md bg-slate-300 px-4 py-2 text-sm font-semibold text-slate-600 cursor-not-allowed">
          Save draft (connect backend)
        </button>
      </form>
    </div>
  );
}

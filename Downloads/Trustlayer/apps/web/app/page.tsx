import Link from "next/link";

export default function HomePage() {
  return (
    <>
      <section className="border-b border-slate-200 bg-white">
        <div className="trust-container py-16 sm:py-24">
          <p className="text-sm font-semibold tracking-wide text-trust-700 uppercase">TRUSTLAYER</p>
          <h1 className="mt-3 text-4xl font-semibold tracking-tight text-ink-900 sm:text-5xl max-w-3xl">
            Trade with confidence.
          </h1>
          <p className="mt-6 text-lg text-slate-600 max-w-2xl">
            Agree on the terms. Secure the payment in Solana USDC escrow. Complete the deal with evidence
            and clear settlement rules. Built for direct transactions that start on WhatsApp, social media,
            or referrals.
          </p>
          <div className="mt-10 flex flex-wrap gap-4">
            <Link
              href="/app/transactions/new"
              className="inline-flex items-center justify-center rounded-md bg-trust-700 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-trust-800 focus:outline-none focus:ring-2 focus:ring-trust-500 focus:ring-offset-2"
            >
              Create a protected transaction
            </Link>
            <Link
              href="/how-it-works"
              className="inline-flex items-center justify-center rounded-md border border-slate-300 bg-white px-6 py-3 text-sm font-semibold text-ink-800 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-trust-500 focus:ring-offset-2"
            >
              See how it works
            </Link>
          </div>
        </div>
      </section>
      <section className="trust-container py-16">
        <h2 className="text-2xl font-semibold text-ink-900">How it works</h2>
        <div className="mt-8 grid gap-8 sm:grid-cols-3">
          <div className="rounded-lg border border-slate-200 bg-white p-6 shadow-sm">
            <p className="text-xs font-semibold text-trust-700 uppercase tracking-wide">Step 1</p>
            <h3 className="mt-2 text-lg font-semibold text-ink-900">Agree</h3>
            <p className="mt-2 text-sm text-slate-600">
              Buyer and seller confirm terms, amount in USDC, deadlines, and acceptance criteria before
              funding.
            </p>
          </div>
          <div className="rounded-lg border border-slate-200 bg-white p-6 shadow-sm">
            <p className="text-xs font-semibold text-trust-700 uppercase tracking-wide">Step 2</p>
            <h3 className="mt-2 text-lg font-semibold text-ink-900">Secure</h3>
            <p className="mt-2 text-sm text-slate-600">
              Buyer funds a non-custodial Solana escrow (program-controlled vault). Seller cannot withdraw
              directly.
            </p>
          </div>
          <div className="rounded-lg border border-slate-200 bg-white p-6 shadow-sm">
            <p className="text-xs font-semibold text-trust-700 uppercase tracking-wide">Step 3</p>
            <h3 className="mt-2 text-lg font-semibold text-ink-900">Complete</h3>
            <p className="mt-2 text-sm text-slate-600">
              Buyer releases after acceptance, or a disclosed dispute process handles outcomes under the
              published policy.
            </p>
          </div>
        </div>
      </section>
      <section className="border-t border-slate-200 bg-slate-100/50">
        <div className="trust-container py-16">
          <h2 className="text-2xl font-semibold text-ink-900">Built for clarity and safety</h2>
          <ul className="mt-6 grid gap-4 sm:grid-cols-2 text-sm text-slate-700 list-disc list-inside max-w-4xl">
            <li>No custody by TRUSTLAYER: you sign with your own wallet.</li>
            <li>USDC on Solana only in MVP (with clear network fee guidance).</li>
            <li>Private evidence stored securely; not written to public chain data.</li>
            <li>Disputes are human-reviewed under a published policy—AI assists summaries only.</li>
          </ul>
          <p className="mt-6 text-sm text-slate-600">
            <Link className="text-trust-700 underline" href="/security">Security model</Link> ·{" "}
            <Link className="text-trust-700 underline" href="/fees">Fees</Link> ·{" "}
            <Link className="text-trust-700 underline" href="/risk-disclosure">Risk disclosure</Link>
          </p>
        </div>
      </section>
    </>
  );
}

import Link from "next/link";

export default function AppDashboardPage() {
  return (
    <div className="trust-container py-10">
      <h1 className="text-2xl font-semibold text-ink-900">Dashboard</h1>
      <p className="mt-2 text-slate-600 text-sm">Sign in to view your transactions. This area requires Supabase authentication (to be connected).</p>
      <div className="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <Link href="/app/transactions" className="rounded-lg border border-slate-200 bg-white p-6 hover:border-trust-300">
          <h2 className="font-semibold text-ink-900">Transactions</h2>
          <p className="mt-1 text-sm text-slate-600">Create, review, and track protected transactions.</p>
        </Link>
        <Link href="/app/transactions/new" className="rounded-lg border border-slate-200 bg-white p-6 hover:border-trust-300">
          <h2 className="font-semibold text-ink-900">New transaction</h2>
          <p className="mt-1 text-sm text-slate-600">Start a new agreement draft.</p>
        </Link>
        <Link href="/app/wallet" className="rounded-lg border border-slate-200 bg-white p-6 hover:border-trust-300">
          <h2 className="font-semibold text-ink-900">Wallet</h2>
          <p className="mt-1 text-sm text-slate-600">Connect and verify your Solana wallet.</p>
        </Link>
      </div>
    </div>
  );
}

import Link from "next/link";

export function SiteFooter() {
  return (
    <footer className="border-t border-slate-200 bg-white">
      <div className="trust-container py-12 grid gap-8 sm:grid-cols-2 lg:grid-cols-4 text-sm">
        <div>
          <p className="font-semibold text-ink-900">TRUSTLAYER</p>
          <p className="mt-2 text-slate-600">Trade with confidence.</p>
          <p className="mt-3 text-xs text-slate-500">
            Not a marketplace. Non-custodial escrow on Solana (USDC). Not legal or financial advice.
          </p>
        </div>
        <div>
          <p className="font-semibold text-ink-900">Product</p>
          <ul className="mt-2 space-y-2 text-slate-600">
            <li><Link href="/how-it-works" className="hover:text-ink-900">How it works</Link></li>
            <li><Link href="/security" className="hover:text-ink-900">Security</Link></li>
            <li><Link href="/fees" className="hover:text-ink-900">Fees</Link></li>
          </ul>
        </div>
        <div>
          <p className="font-semibold text-ink-900">Policies</p>
          <ul className="mt-2 space-y-2 text-slate-600">
            <li><Link href="/terms" className="hover:text-ink-900">Terms</Link></li>
            <li><Link href="/privacy" className="hover:text-ink-900">Privacy</Link></li>
            <li><Link href="/risk-disclosure" className="hover:text-ink-900">Risk disclosure</Link></li>
            <li><Link href="/disputes" className="hover:text-ink-900">Dispute policy</Link></li>
          </ul>
        </div>
        <div>
          <p className="font-semibold text-ink-900">Support</p>
          <ul className="mt-2 space-y-2 text-slate-600">
            <li><Link href="/support" className="hover:text-ink-900">Support</Link></li>
            <li><Link href="/status" className="hover:text-ink-900">Status</Link></li>
          </ul>
        </div>
      </div>
      <div className="border-t border-slate-200 py-6 text-center text-xs text-slate-500">
        © {new Date().getFullYear()} TRUSTLAYER. Draft materials require legal review before production launch.
      </div>
    </footer>
  );
}

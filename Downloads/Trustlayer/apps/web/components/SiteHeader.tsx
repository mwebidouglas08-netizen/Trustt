import Link from "next/link";

const nav = [
  { href: "/how-it-works", label: "How it works" },
  { href: "/security", label: "Security" },
  { href: "/fees", label: "Fees" },
  { href: "/disputes", label: "Disputes" },
  { href: "/support", label: "Support" },
];

export function SiteHeader() {
  return (
    <header className="border-b border-slate-200 bg-white">
      <div className="trust-container flex h-16 items-center justify-between gap-4">
        <Link href="/" className="text-lg font-semibold tracking-tight text-ink-900">
          TRUSTLAYER
        </Link>
        <nav className="hidden md:flex items-center gap-6 text-sm text-slate-700">
          {nav.map((item) => (
            <Link key={item.href} href={item.href} className="hover:text-ink-900">
              {item.label}
            </Link>
          ))}
        </nav>
        <div className="flex items-center gap-3">
          <Link
            href="/app"
            className="text-sm font-medium text-slate-700 hover:text-ink-900"
          >
            Sign in
          </Link>
          <Link
            href="/app/transactions/new"
            className="rounded-md bg-trust-700 px-4 py-2 text-sm font-semibold text-white hover:bg-trust-800"
          >
            New transaction
          </Link>
        </div>
      </div>
    </header>
  );
}

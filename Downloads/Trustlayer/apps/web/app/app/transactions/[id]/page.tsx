type Props = { params: { id: string } };

export default function TransactionDetailPage({ params }: Props) {
  return (
    <div className="trust-container py-10">
      <h1 className="text-2xl font-semibold text-ink-900">Transaction</h1>
      <p className="mt-2 text-sm text-slate-600">ID: {params.id}</p>
      <p className="mt-4 text-sm text-slate-600">Detail view, timeline, evidence, and chain verification will load from Supabase + Solana RPC after integration.</p>
    </div>
  );
}

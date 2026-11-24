import { NextResponse } from "next/server";

export async function GET() {
  return NextResponse.json({
    ok: true,
    service: "trustlayer-web",
    timestamp: new Date().toISOString(),
    note: "No secrets exposed. Configure Supabase/Solana env vars for full functionality.",
  });
}

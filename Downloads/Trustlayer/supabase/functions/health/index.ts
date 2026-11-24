import { serve } from "https://deno.land/std@0.224.0/http/server.ts";

serve((_req) => {
  return new Response(JSON.stringify({ ok: true, fn: "health" }), {
    headers: { "Content-Type": "application/json" },
  });
});

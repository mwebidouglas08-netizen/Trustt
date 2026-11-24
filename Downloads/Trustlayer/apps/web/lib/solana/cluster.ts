export function getSolanaCluster(): "devnet" | "testnet" | "mainnet-beta" {
  const c = process.env.NEXT_PUBLIC_SOLANA_CLUSTER;
  if (c === "testnet" || c === "mainnet-beta") return c;
  return "devnet";
}

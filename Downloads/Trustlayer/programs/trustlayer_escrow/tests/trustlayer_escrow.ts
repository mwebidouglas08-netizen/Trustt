import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("trustlayer_escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("loads provider", () => {
    assert.ok(provider.wallet.publicKey);
  });

  // Full escrow flow tests require a USDC mint and token accounts; add in CI with local validator + spl-token setup.
});

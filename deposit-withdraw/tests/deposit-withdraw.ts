import * as anchor from "@coral-xyz/anchor";
import { Program} from "@coral-xyz/anchor";
import { DepositWithdraw } from "../target/types/deposit_withdraw";
import { assert, expect } from "chai";
import { BN } from "bn.js";

describe("deposit-withdraw", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DepositWithdraw as Program<DepositWithdraw>;

  it("Should initialize successfully", async () => {
    // TODO: add code to capture event emition and assure program initialization
    await program.methods.initialize().rpc();

    const vaults = await program.account.vault.all();
    expect(vaults.length).to.be.greaterThan(0);
  });

  // Why the hell it doesnt accept bigint? Why it uses BigNumber?
  it("Should deposit successfully", async () => {
    const signer = anchor.getProvider().publicKey;
    const amount = 200000000;
    const oldBalance = await anchor.getProvider().connection.getBalance(signer);

    const vault = await program.account.vault.all();
    console.log(vault[0].publicKey)

      // TODO: get this info from LAMPORTS per sol constant
      await program.methods.deposit(new BN(amount)).accounts({ vault: vault[0].publicKey, signer }).rpc();

      const newBalance = await anchor.getProvider().connection.getBalance(signer);

      expect(oldBalance - newBalance).to.be.greaterThanOrEqual(amount);
  });

  it("Should emit error when a deposit is less than 0.1 SOL", async () => {

  });

  it("Should withdraw successfully", async () => {

  });
});

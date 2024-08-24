import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TinyAdventure } from "../target/types/tiny_adventure";
import { expect } from "chai";

describe("tiny_adventure", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TinyAdventure as Program<TinyAdventure>;
  let gameStatePubKey: anchor.web3.PublicKey;

  it("Should initialize gameDataAccount with playerPosition in 0", async () => {
    // Add your test here.
    await program.methods.initialize().rpc();
    const gameState = (await program.account.gameDataAccount.all()).at(0);
    gameStatePubKey = gameState.publicKey;

    expect(gameState.account.playerPosition).to.be.eql(0);
  });

  it("Should add 1 to playerPosition when move_right is called", async () => {
    await program.methods.moveRight().accounts({gameDataAccount: gameStatePubKey}).rpc();
    const gameState = (await program.account.gameDataAccount.all()).at(0);

    expect(gameState.account.playerPosition).to.be.eql(1);
  });

  it("Should decrease 1 to playerPosition when move_left is called", async () => {
    await program.methods.moveLeft().accounts({gameDataAccount: gameStatePubKey}).rpc();
    const gameState = (await program.account.gameDataAccount.all()).at(0);

    expect(gameState.account.playerPosition).to.be.eql(0);
  });
});

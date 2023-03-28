import {
    it,
    describe,
} from 'mocha'
import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    Transaction,
} from '@solana/web3.js'
import { createTestInstruction, TestInstruction } from './instruction'


function loadKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
}

describe("Nautilus Program Unit Tests", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed')
    const payer = loadKeypairFromFile(require('os').homedir() + '/.config/solana/id.json')
    const program = loadKeypairFromFile('./program/target/deploy/program_nautilus-keypair.json')

    async function test(instruction: TestInstruction) {
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(createTestInstruction(
                payer.publicKey,
                program.publicKey,
                instruction,
            )),
            [payer]
        )
    }

    it("Test Wallets", async () => test(TestInstruction.Wallets))
    it("Test Mints", async () => test(TestInstruction.Mints))
    it("Test Metadatas", async () => test(TestInstruction.Metadatas))
    it("Test Associated Token Accounts", async () => test(TestInstruction.AssociatedTokens))
    it("Test Tokens", async () => test(TestInstruction.Tokens))
  })
  
import {
    it,
    describe,
} from 'mocha'
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
} from '@solana/web3.js'
import { createTestInstruction, TestInstruction } from './instruction'
import { CONNECTION, PAYER, PROGRAM_SOURCE_BASIC } from '../const'

describe("Nautilus Unit Tests: Source Basic", async () => {

    const connection = CONNECTION
    const payer = PAYER
    const program = PROGRAM_SOURCE_BASIC
    
    const from = Keypair.generate()
    const to = Keypair.generate()

    async function initAccount(publicKey: PublicKey) {
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL)
        )
    }

    async function initTestAccounts() {
        initAccount(from.publicKey)
        initAccount(to.publicKey)
    }

    async function test(instruction: TestInstruction) {
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(createTestInstruction(
                from.publicKey,
                to.publicKey,
                payer.publicKey,
                program.publicKey,
                instruction,
            )),
            [payer],
            {skipPreflight: true}
        )
    }

    before(async () => initTestAccounts())
    it("Test Wallets", async () => test(TestInstruction.Wallets))
    it("Test Mints", async () => test(TestInstruction.Mints))
    it("Test Metadatas", async () => test(TestInstruction.Metadatas))
    it("Test Associated Token Accounts", async () => test(TestInstruction.AssociatedTokens))
    it("Test Tokens", async () => test(TestInstruction.Tokens))
  })
  
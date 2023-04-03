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
    TransactionInstruction,
} from '@solana/web3.js'
import { CONNECTION, PAYER, PROGRAM_SOURCE_ROBUST } from '../const'
import { 
    createCreateTokenInstruction, 
    createGetTokenInfoInstruction, 
} from './instructions'

describe("Nautilus Unit Tests: Source Robust", async () => {

    const skipMetadata = false; // Enabled for localnet

    const connection = CONNECTION
    const payer = PAYER
    const program = PROGRAM_SOURCE_ROBUST
    
    const rent_payer = Keypair.generate()

    const newTokenMint = Keypair.generate()

    const decimals = 9
    const title = "Nautilus Token"
    const symbol = "NTLS"
    const uri = "NTLS"

    async function initAccount(publicKey: PublicKey) {
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL)
        )
    }

    async function initTestAccounts() {
        initAccount(rent_payer.publicKey)
    }

    async function test(ix: TransactionInstruction, signers: Keypair[]) {
        let sx = await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            signers,
            {skipPreflight: true}
        )
        console.log(`\n\n  [INFO]: sig: ${sx}\n`)
    }

    before(async () => {
        if (skipMetadata) console.log("  [WARN]: `skipMetadata` is set to `true`, so tests for Metadata and Token will not execute & automatically pass.")
        initTestAccounts()
    })

    it("Create Token", async () => {if (!skipMetadata) return test(
        createCreateTokenInstruction(newTokenMint.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMint],
    )})

    it("Get Token Info", async () => {if (!skipMetadata) return test(
        createGetTokenInfoInstruction(newTokenMint.publicKey, payer.publicKey, program.publicKey),
        [payer, newTokenMint],
    )})
  })
  
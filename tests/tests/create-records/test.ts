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
import { PAYER, PROGRAM_SOURCE_ROBUST, TEST_CONFIGS } from '../const'
import { 
    createCreateCarInstruction,
    createCreateHomeInstruction,
    createCreatePersonInstruction,
    createCreateTokenInstruction, 
    createInitializeInstruction, 
    createReadCarInstruction,
    createReadHomeInstruction,
    createReadPersonInstruction,
    createReadTokenInstruction, 
} from './instructions'

describe("Nautilus Unit Tests: Create Records", async () => {

    const skipMetadata = TEST_CONFIGS.skipMetadata // Enabled for localnet

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_SOURCE_ROBUST
    
    const rent_payer = Keypair.generate()

    const newTokenMint = Keypair.generate()

    const decimals = 9
    const title = "Nautilus Token"
    const symbol = "NTLS"
    const uri = "NTLS"

    const personName = "Joe"
    const homeId = 1
    const homeHouseNumber = 15
    const homeStreet = "Solana St."
    const carMake = "Chevrolet"
    const carModel = "Corvette"

    async function initAccount(publicKey: PublicKey) {
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL)
        )
    }

    async function initTestAccounts() {
        initAccount(rent_payer.publicKey)
    }

    async function test(ix: TransactionInstruction, signers: Keypair[]) {
        TEST_CONFIGS.sleep()
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

    it("Read Token", async () => {if (!skipMetadata) return test(
        createReadTokenInstruction(newTokenMint.publicKey, program.publicKey),
        [payer],
    )})

    it("Initialize Nautilus Index", async () => test(
        createInitializeInstruction(payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Person", async () => test(
        await createCreatePersonInstruction(payer.publicKey, program.publicKey, personName, payer.publicKey),
        [payer],
    ))

    it("Read Person", async () => test(
        await createReadPersonInstruction(program.publicKey),
        [payer],
    ))

    it("Create Home", async () => test(
        createCreateHomeInstruction(payer.publicKey, program.publicKey, homeId, homeHouseNumber, homeStreet),
        [payer],
    ))

    it("Read Home", async () => test(
        createReadHomeInstruction(program.publicKey, homeId),
        [payer],
    ))

    it("Create Car", async () => test(
        await createCreateCarInstruction(payer.publicKey, program.publicKey, carMake, carModel, payer.publicKey, payer.publicKey),
        [payer],
    ))

    it("Read Car", async () => test(
        await createReadCarInstruction(program.publicKey),
        [payer],
    ))
  })
  
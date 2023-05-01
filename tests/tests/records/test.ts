import {
    it,
    describe,
} from 'mocha'
import {
    Keypair,
    sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js'
import { PAYER, PROGRAM_RECORDS, TEST_CONFIGS } from '../const'
import { 
    createCreateCarInstruction,
    createCreateHomeInstruction,
    createCreatePersonInstruction, 
    createInitializeInstruction, 
    createReadCarInstruction,
    createReadHomeInstruction,
    createReadPersonInstruction,
} from './instructions'

describe("Nautilus Unit Tests: Create Records", async () => {

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_RECORDS

    const personName = "Joe"
    const homeId = 1
    const homeHouseNumber = 15
    const homeStreet = "Solana St."
    const carMake = "Chevrolet"
    const carModel = "Corvette"

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
  
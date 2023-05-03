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
import { PAYER, PROGRAM_ACCOUNTS, TEST_CONFIGS } from '../const'
import { 
    createCreateCarInstruction,
    createCreateHomeInstruction,
    createCreatePersonInstruction, 
    createReadCarInstruction,
    createReadHomeInstruction,
    createReadPersonInstruction,
} from './instructions'

describe("Nautilus Unit Tests: Create Accounts", async () => {

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_ACCOUNTS

    const personName = "Joe"
    const personAuthority = Keypair.generate().publicKey
    const homeHouseNumber = 15
    const homeStreet = "Solana St."
    const homeSomeRandomPubkey = Keypair.generate().publicKey
    const carMake = "Chevrolet"
    const carModel = "Corvette"
    const carPurchaseAuthority = Keypair.generate().publicKey
    const carOperatingAuthority = Keypair.generate().publicKey

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

    it("Create Person", async () => test(
        await createCreatePersonInstruction(payer.publicKey, program.publicKey, personName, personAuthority),
        [payer],
    ))

    it("Read Person", async () => test(
        await createReadPersonInstruction(program.publicKey, personAuthority),
        [payer],
    ))

    it("Create Home", async () => test(
        createCreateHomeInstruction(payer.publicKey, program.publicKey, homeHouseNumber, homeStreet, homeSomeRandomPubkey),
        [payer],
    ))

    it("Read Home", async () => test(
        createReadHomeInstruction(program.publicKey, homeSomeRandomPubkey),
        [payer],
    ))

    it("Create Car", async () => test(
        await createCreateCarInstruction(payer.publicKey, program.publicKey, carMake, carModel, carPurchaseAuthority, carOperatingAuthority),
        [payer],
    ))

    it("Read Car", async () => test(
        await createReadCarInstruction(program.publicKey, carPurchaseAuthority, carOperatingAuthority),
        [payer],
    ))
  })
  
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
import { PAYER, PROGRAM_WALLETS, TEST_CONFIGS } from '../const'
import { 
    createAllocateWalletInstruction,
    createComplexInstruction,
    createCreateWalletInstruction, 
    createCreateWalletWithPayerInstruction,
    createReadWalletInstruction,
    createTransferInstruction,
} from './instructions'
import { createAssignInstruction } from './instructions/assign'

describe("Nautilus Unit Tests: Wallets", async () => {

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_WALLETS
    
    const rent_payer = Keypair.generate()

    const newWallet = Keypair.generate()
    const newWalletToBeAllocated = Keypair.generate()
    const newWalletToBeAssignedAway = Keypair.generate()
    const newWalletWithPayer = Keypair.generate()
    const transferAmount = LAMPORTS_PER_SOL / 1000

    // Complex instruction
    const compAuthority1 = Keypair.generate()
    const compAuthority2 = Keypair.generate()
    const compRentPayer1 = Keypair.generate()
    const compRentPayer2 = Keypair.generate()
    const compTransferRecipient = Keypair.generate()
    const compWalletToAllocate = Keypair.generate()
    const compWalletToCreate = Keypair.generate()
    const compWalletToCreateWithTransferSafe = Keypair.generate()
    const compWalletToCreateWithTransferUnsafe = Keypair.generate()
    const compFundAmount = LAMPORTS_PER_SOL / 1000
    const compTransferAmount = LAMPORTS_PER_SOL / 1000

    async function initAccount(publicKey: PublicKey) {
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL)
        )
    }

    async function initTestAccounts() {
        initAccount(rent_payer.publicKey)
        initAccount(compRentPayer1.publicKey)
        initAccount(compRentPayer2.publicKey)
        initAccount(compAuthority2.publicKey)
    }

    async function test(ix: TransactionInstruction, signers: Keypair[]) {
        await TEST_CONFIGS.sleep()
        let sx = await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            signers,
            {skipPreflight: true}
        )
        console.log(`\n\n  [INFO]: sig: ${sx}\n`)
    }

    before(async () => {
        await TEST_CONFIGS.sleep()
        initTestAccounts()
    })

    // Wallets

    it("Allocate Wallet", async () => test(
        createAllocateWalletInstruction(newWalletToBeAllocated.publicKey, payer.publicKey, program.publicKey),
        [payer, newWalletToBeAllocated],
    ))

    it("Create Wallet to be assigned away", async () => test(
        createCreateWalletInstruction(newWalletToBeAssignedAway.publicKey, payer.publicKey, program.publicKey),
        [payer, newWalletToBeAssignedAway],
    ))

    it("Assign away Wallet", async () => test(
        createAssignInstruction(newWalletToBeAssignedAway.publicKey, program.publicKey, program.publicKey),
        [payer, newWalletToBeAssignedAway],
    ))

    it("Create Wallet", async () => test(
        createCreateWalletInstruction(newWallet.publicKey, payer.publicKey, program.publicKey),
        [payer, newWallet],
    ))

    it("Read Wallet", async () => test(
        createReadWalletInstruction(newWallet.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Wallet with Payer", async () => test(
        createCreateWalletWithPayerInstruction(newWalletWithPayer.publicKey, payer.publicKey, program.publicKey),
        [payer, newWalletWithPayer],
    ))

    it("Read Wallet Created With Payer", async () => test(
        createReadWalletInstruction(newWalletWithPayer.publicKey, program.publicKey),
        [payer],
    ))

    it("Transfer", async () => test(
        createTransferInstruction(payer.publicKey, newWallet.publicKey, program.publicKey, transferAmount),
        [payer],
    ))

    it("Complex", async () => test(
        createComplexInstruction(
            compAuthority1.publicKey, 
            compAuthority2.publicKey, 
            compRentPayer1.publicKey, 
            compRentPayer2.publicKey,
            compTransferRecipient.publicKey,
            compWalletToAllocate.publicKey,
            compWalletToCreate.publicKey,
            compWalletToCreateWithTransferSafe.publicKey,
            compWalletToCreateWithTransferUnsafe.publicKey,
            payer.publicKey,
            program.publicKey,
            compFundAmount,
            compTransferAmount,
        ),
        [
            payer,
            compAuthority1,
            compAuthority2,
            compRentPayer1,
            compRentPayer2,
            compWalletToAllocate,
            compWalletToCreate,
        ],
    ))
  })
  
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
import { PAYER, PROGRAM_SOURCE_CREATE, TEST_CONFIGS } from '../const'
import { 
    createCreateAssociatedTokenInstruction,
    createCreateAssociatedTokenWithPayerInstruction,
    createCreateMetadataInstruction,
    createCreateMetadataWithPayerInstruction,
    createCreateMintInstruction, 
    createCreateMintWithPayerInstruction, 
    createCreateTokenInstruction, 
    createCreateTokenWithPayerInstruction, 
    createCreateWalletInstruction, 
    createCreateWalletWithPayerInstruction,
    createTransferWalletInstruction,
} from './instructions'

describe("Nautilus Unit Tests: Source Create", async () => {

    const skipMetadata = TEST_CONFIGS.skipMetadata // `true` for localnet

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_SOURCE_CREATE
    
    const rent_payer = Keypair.generate()

    const newWallet = Keypair.generate()
    const newWalletWithPayer = Keypair.generate()
    const newMint = Keypair.generate()
    const newMintWithPayer = Keypair.generate()
    const newTokenMint = Keypair.generate()
    const newTokenMintWithPayer = Keypair.generate()

    const decimals = 9
    const title = "Nautilus Token"
    const symbol = "NTLS"
    const uri = "NTLS"

    const transferAmount = LAMPORTS_PER_SOL / 100

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

    it("Create Wallet", async () => test(
        createCreateWalletInstruction(newWallet.publicKey, payer.publicKey, program.publicKey),
        [payer, newWallet],
    ))

    it("Create Wallet with Payer", async () => test(
        createCreateWalletWithPayerInstruction(newWalletWithPayer.publicKey, payer.publicKey, program.publicKey),
        [payer, newWalletWithPayer],
    ))

    it("Create Mint", async () => test(
        createCreateMintInstruction(newMint.publicKey, payer.publicKey, program.publicKey, decimals),
        [payer, newMint],
    ))

    it("Create Mint with Payer", async () => test(
        createCreateMintWithPayerInstruction(newMintWithPayer.publicKey, payer.publicKey, program.publicKey, decimals),
        [payer, newMintWithPayer],
    ))

    it("Create Metadata", async () => {if (!skipMetadata) return test(
        createCreateMetadataInstruction(newMint.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    )})

    it("Create Metadata with Payer", async () => {if (!skipMetadata) return test(
        createCreateMetadataWithPayerInstruction(newMintWithPayer.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    )})

    it("Create Associated Token", async () => test(
        createCreateAssociatedTokenInstruction(newMint.publicKey, newWallet.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Associated Token with Payer", async () => test(
        createCreateAssociatedTokenWithPayerInstruction(newMintWithPayer.publicKey, newWalletWithPayer.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Token", async () => {if (!skipMetadata) return test(
        createCreateTokenInstruction(newTokenMint.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMint],
    )})

    it("Create Token with Payer", async () => {if (!skipMetadata) return test(
        createCreateTokenWithPayerInstruction(newTokenMintWithPayer.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMintWithPayer],
    )})

    it("Transfer Wallet", async () => test(
        createTransferWalletInstruction(payer.publicKey, newWallet.publicKey, program.publicKey, transferAmount),
        [payer],
    ))
  })
  
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
    createReadAssociatedTokenCreatedWithPayerInstruction,
    createReadAssociatedTokenInstruction,
    createReadMetadataCreatedWithPayerInstruction,
    createReadMetadataInstruction,
    createReadMintCreatedWithPayerInstruction,
    createReadMintInstruction,
    createReadTokenCreatedWithPayerInstruction,
    createReadTokenInstruction,
    createReadWalletCreatedWithPayerInstruction,
    createReadWalletInstruction,
    createTransferWalletInstruction,
} from './instructions'

describe("Nautilus Unit Tests: Create Source", async () => {

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
        if (skipMetadata) console.log("  [WARN]: `skipMetadata` is set to `true`, so tests for Metadata and Token will not execute & will automatically pass.")
        await TEST_CONFIGS.sleep()
        initTestAccounts()
    })

    // Wallets

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
        createReadWalletCreatedWithPayerInstruction(newWalletWithPayer.publicKey, program.publicKey),
        [payer],
    ))

    // Mints

    it("Create Mint", async () => test(
        createCreateMintInstruction(newMint.publicKey, payer.publicKey, program.publicKey, decimals),
        [payer, newMint],
    ))

    it("Read Mint", async () => test(
        createReadMintInstruction(newMint.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Mint with Payer", async () => test(
        createCreateMintWithPayerInstruction(newMintWithPayer.publicKey, payer.publicKey, program.publicKey, decimals),
        [payer, newMintWithPayer],
    ))

    it("Read Mint Created With Payer", async () => test(
        createReadMintCreatedWithPayerInstruction(newMintWithPayer.publicKey, program.publicKey),
        [payer],
    ))

    // Metadatas

    it("Create Metadata", async () => {if (!skipMetadata) return test(
        createCreateMetadataInstruction(newMint.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    )})

    it("Read Metadata", async () => {if (!skipMetadata) return test(
        createReadMetadataInstruction(newMint.publicKey, program.publicKey),
        [payer],
    )})

    it("Create Metadata with Payer", async () => {if (!skipMetadata) return test(
        createCreateMetadataWithPayerInstruction(newMintWithPayer.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    )})

    it("Read Metadata Created With Payer", async () => {if (!skipMetadata) return test(
        createReadMetadataCreatedWithPayerInstruction(newMintWithPayer.publicKey, program.publicKey),
        [payer],
    )})

    // Associated Token Accounts

    it("Create Associated Token", async () => test(
        createCreateAssociatedTokenInstruction(newMint.publicKey, newWallet.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Read Associated Token", async () => test(
        createReadAssociatedTokenInstruction(newMint.publicKey, newWallet.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Associated Token with Payer", async () => test(
        createCreateAssociatedTokenWithPayerInstruction(newMintWithPayer.publicKey, newWalletWithPayer.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Read Associated Token Created With Payer", async () => test(
        createReadAssociatedTokenCreatedWithPayerInstruction(newMintWithPayer.publicKey, newWalletWithPayer.publicKey, program.publicKey),
        [payer],
    ))

    // Tokens

    it("Create Token", async () => {if (!skipMetadata) return test(
        createCreateTokenInstruction(newTokenMint.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMint],
    )})

    it("Read Token", async () => {if (!skipMetadata) return test(
        createReadTokenInstruction(newMint.publicKey, program.publicKey),
        [payer],
    )})

    it("Create Token with Payer", async () => {if (!skipMetadata) return test(
        createCreateTokenWithPayerInstruction(newTokenMintWithPayer.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMintWithPayer],
    )})

    it("Read Token Created With Payer", async () => {if (!skipMetadata) return test(
        createReadTokenCreatedWithPayerInstruction(newMintWithPayer.publicKey, program.publicKey),
        [payer],
    )})

    // Transfers

    it("Transfer Wallet", async () => test(
        createTransferWalletInstruction(payer.publicKey, newWallet.publicKey, program.publicKey, transferAmount),
        [payer],
    ))
  })
  
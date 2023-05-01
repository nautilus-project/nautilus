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
import { PAYER, PROGRAM_TOKENS, TEST_CONFIGS } from '../const'
import { 
    MyInstructions,
    createBurnTokensInstruction,
    createCreateAssociatedTokenInstruction,
    createCreateAssociatedTokenWithPayerInstruction,
    createCreateMetadataInstruction,
    createCreateMetadataWithPayerInstruction,
    createCreateMintInstruction, 
    createCreateMintWithPayerInstruction, 
    createCreateTokenInstruction, 
    createCreateTokenWithPayerInstruction, 
    createFreezeAssociatedTokenInstruction, 
    createMintDisableMintingInstruction, 
    createMintMintToInstruction, 
    createReadAssociatedTokenInstruction,
    createReadMetadataInstruction,
    createReadMintInstruction,
    createReadTokenInstruction,
    createThawAssociatedTokenInstruction,
    createTokenDisableMintingInstruction,
    createTokenMintToInstruction,
    createTransferTokensInstruction,
} from './instructions'

describe("Nautilus Unit Tests: Tokens", async () => {

    const skipMetadata = TEST_CONFIGS.skipMetadata // `true` for localnet

    const connection = TEST_CONFIGS.connection
    const payer = PAYER
    const program = PROGRAM_TOKENS
    
    const rent_payer = Keypair.generate()

    const testWallet1 = Keypair.generate()
    const testWallet2 = Keypair.generate()

    const newMint = Keypair.generate()
    const newMintWithPayer = Keypair.generate()
    const mintMintAmount = 20
    const mintTransferAmount = 5
    const mintBurnAmount = 5
    
    const newTokenMint = Keypair.generate()
    const newTokenMintWithPayer = Keypair.generate()
    const tokenMintAmount = 20
    const tokenTransferAmount = 5
    const tokenBurnAmount = 5

    const decimals = 9
    const title = "Nautilus Token"
    const symbol = "NTLS"
    const uri = "NTLS"

    async function initAccount(publicKey: PublicKey) {
        await TEST_CONFIGS.sleep()
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL / 100)
        )
    }

    async function initTestAccounts() {
        initAccount(rent_payer.publicKey)
        initAccount(testWallet1.publicKey)
        initAccount(testWallet2.publicKey)
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
        createReadMintInstruction(newMintWithPayer.publicKey, program.publicKey),
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
        createReadMetadataInstruction(newMintWithPayer.publicKey, program.publicKey),
        [payer],
    )})

    // Associated Token Accounts

    it("Create Associated Token", async () => test(
        createCreateAssociatedTokenInstruction(newMint.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Read Associated Token", async () => test(
        createReadAssociatedTokenInstruction(newMint.publicKey, testWallet1.publicKey, program.publicKey),
        [payer],
    ))

    it("Freeze Associated Token", async () => test(
        createFreezeAssociatedTokenInstruction(newMint.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Thaw Associated Token", async () => test(
        createThawAssociatedTokenInstruction(newMint.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Associated Token with Payer", async () => test(
        createCreateAssociatedTokenWithPayerInstruction(newMintWithPayer.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Read Associated Token Created With Payer", async () => test(
        createReadAssociatedTokenInstruction(newMintWithPayer.publicKey, testWallet1.publicKey, program.publicKey),
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
        createReadTokenInstruction(newMintWithPayer.publicKey, program.publicKey),
        [payer],
    )})

    // Minting & Transferring

    it("Mint: Mint To", async () => test(
        createMintMintToInstruction(newMint.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey, mintMintAmount),
        [payer],
    ))

    it("Mint: Burn", async () => test(
        createBurnTokensInstruction(newMint.publicKey, testWallet1.publicKey, program.publicKey, mintBurnAmount),
        [payer, testWallet1],
    ))

    it("Create Associated Token For Transfer", async () => test(
        createCreateAssociatedTokenInstruction(newMint.publicKey, testWallet2.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Mint: Transfer", async () => test(
        createTransferTokensInstruction(newMint.publicKey, testWallet1.publicKey, testWallet2.publicKey, program.publicKey, mintTransferAmount),
        [payer, testWallet1],
    ))

    it("Mint: Disable Minting", async () => test(
        createMintDisableMintingInstruction(MyInstructions.MintDisableMinting, newMint.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Token: Mint To", async () => {if (!skipMetadata) return test(
        createTokenMintToInstruction(newTokenMint.publicKey, testWallet1.publicKey, payer.publicKey, program.publicKey, tokenMintAmount),
        [payer],
    )})

    it("Token: Burn", async () => {if (!skipMetadata) return test(
        createBurnTokensInstruction(newTokenMint.publicKey, testWallet1.publicKey, program.publicKey, tokenBurnAmount),
        [payer, testWallet1],
    )})

    it("Create Associated Token For Transfer", async () => {if (!skipMetadata) return test(
        createCreateAssociatedTokenInstruction(newTokenMint.publicKey, testWallet2.publicKey, payer.publicKey, program.publicKey),
        [payer],
    )})

    it("Token: Transfer", async () => {if (!skipMetadata) return test(
        createTransferTokensInstruction(newTokenMint.publicKey, testWallet1.publicKey, testWallet2.publicKey, program.publicKey, tokenTransferAmount),
        [payer, testWallet1],
    )})

    it("Token: Disable Minting", async () => {if (!skipMetadata) return test(
        createTokenDisableMintingInstruction(MyInstructions.TokenDisableMinting, newTokenMint.publicKey, payer.publicKey, program.publicKey),
        [payer],
    )})
  })
  
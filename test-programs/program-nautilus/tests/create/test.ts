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
import { CONNECTION, PAYER, PROGRAM_CREATE } from '../const'
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
    createCreateWalletWithPayerInstruction 
} from './instructions'

describe("Nautilus Unit Tests: Create", async () => {

    const connection = CONNECTION
    const payer = PAYER
    const program = PROGRAM_CREATE
    
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

    async function initAccount(publicKey: PublicKey) {
        connection.confirmTransaction(
            await connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL)
        )
    }

    async function initTestAccounts() {
        initAccount(rent_payer.publicKey)
    }

    async function test(ix: TransactionInstruction, signers: Keypair[]) {
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            signers,
            {skipPreflight: true}
        )
    }

    before(async () => initTestAccounts())

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

    it("Create Metadata", async () => test(
        createCreateMetadataInstruction(newMint.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    ))

    it("Create Metadata with Payer", async () => test(
        createCreateMetadataWithPayerInstruction(newMintWithPayer.publicKey, payer.publicKey, program.publicKey, title, symbol, uri),
        [payer],
    ))

    it("Create Associated Token", async () => test(
        createCreateAssociatedTokenInstruction(newMint.publicKey, newWallet.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Associated Token with Payer", async () => test(
        createCreateAssociatedTokenWithPayerInstruction(newMintWithPayer.publicKey, newWalletWithPayer.publicKey, payer.publicKey, program.publicKey),
        [payer],
    ))

    it("Create Token", async () => test(
        createCreateTokenInstruction(newTokenMint.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMint],
    ))

    it("Create Token with Payer", async () => test(
        createCreateTokenWithPayerInstruction(newTokenMintWithPayer.publicKey, payer.publicKey, program.publicKey, decimals, title, symbol, uri),
        [payer, newTokenMintWithPayer],
    ))
  })
  
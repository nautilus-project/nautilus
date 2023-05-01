import * as borsh from "borsh"
import { Buffer } from "buffer"
import { PROGRAM_ID as METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata'
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, MyInstructions } from "."

// Create

class CreateTokenInstructionData {
    instruction: MyInstructions
    decimals: number
    title: string
    symbol: string
    uri: string
    constructor(props: {
        instruction: MyInstructions,
        decimals: number
        title: string,
        symbol: string,
        uri: string,
    }) {
        this.instruction = props.instruction
        this.decimals = props.decimals
        this.title = props.title
        this.symbol = props.symbol
        this.uri = props.uri
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateTokenInstructionDataSchema, this)) 
    }
}

const CreateTokenInstructionDataSchema = new Map([
    [ CreateTokenInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['decimals', 'u8'],
            ['title', 'string'],
            ['symbol', 'string'],
            ['uri', 'string'],
        ],
    }]
])

function getMetadataAddress(mint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
        METADATA_PROGRAM_ID,
    )[0]
}

function createCreateInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    title: string,
    symbol: string,
    uri: string,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateTokenInstructionData({instruction, decimals, title, symbol, uri})

    const newMetadata = getMetadataAddress(newMint)

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateToken) return [
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMint, isSigner: true, isWritable: true},
            {pubkey: newMetadata, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateTokenWithPayer) return [
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMint, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMetadata, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        return []
    }

    const keys = deriveKeys(instruction)

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createCreateTokenInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, decimals, title, symbol, uri, MyInstructions.CreateToken)
}

export function createCreateTokenWithPayerInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, decimals, title, symbol, uri, MyInstructions.CreateTokenWithPayer)
}

// Mint To

class TokenMintToInstructionData {
    instruction: MyInstructions
    amount: number
    constructor(props: {
        instruction: MyInstructions,
        amount: number,
    }) {
        this.instruction = props.instruction
        this.amount = props.amount
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(TokenMintToInstructionDataSchema, this)) 
    }
}

const TokenMintToInstructionDataSchema = new Map([
    [ TokenMintToInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

export function createTokenMintToInstruction(
    mint: PublicKey,
    recipient: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
    amount: number,
): TransactionInstruction {

    const myInstructionObject = new TokenMintToInstructionData({
        instruction: MyInstructions.TokenMintTo, 
        amount,
    })

    const metadata = getMetadataAddress(mint)
    const recipientAssociatedToken = getAssociatedTokenAddressSync(mint, recipient)

    const keys = [
        {pubkey: authority, isSigner: true, isWritable: true},
        {pubkey: recipientAssociatedToken, isSigner: false, isWritable: true},
        {pubkey: mint, isSigner: false, isWritable: true},
        {pubkey: metadata, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

// Disable Minting

export function createTokenDisableMintingInstruction(
    instruction: MyInstructions,
    mint: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
): TransactionInstruction {

    const metadata = getMetadataAddress(mint)

    return createBaseInstruction(
        programId, 
        instruction,
        [
            {pubkey: authority, isSigner: true, isWritable: true},
            {pubkey: mint, isSigner: false, isWritable: true},
            {pubkey: metadata, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

// Read

export function createReadTokenInstruction(
    mint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newMetadata = getMetadataAddress(mint)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadToken,
        [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: newMetadata, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
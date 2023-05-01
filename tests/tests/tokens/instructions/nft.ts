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

class CreateNftInstructionData {
    instruction: MyInstructions
    title: string
    symbol: string
    uri: string
    constructor(props: {
        instruction: MyInstructions,
        title: string,
        symbol: string,
        uri: string,
    }) {
        this.instruction = props.instruction
        this.title = props.title
        this.symbol = props.symbol
        this.uri = props.uri
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateNftInstructionDataSchema, this)) 
    }
}

const CreateNftInstructionDataSchema = new Map([
    [ CreateNftInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
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
    title: string,
    symbol: string,
    uri: string,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateNftInstructionData({instruction, title, symbol, uri})

    const newMetadata = getMetadataAddress(newMint)

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateNft) return [
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMint, isSigner: true, isWritable: true},
            {pubkey: newMetadata, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateNftWithPayer) return [
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

export function createCreateNftInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, title, symbol, uri, MyInstructions.CreateNft)
}

export function createCreateNftWithPayerInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, title, symbol, uri, MyInstructions.CreateNftWithPayer)
}

// Mint To

export function createNftMintToInstruction(
    mint: PublicKey,
    recipient: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const metadata = getMetadataAddress(mint)
    const recipientAssociatedToken = getAssociatedTokenAddressSync(mint, recipient)
    return createBaseInstruction(
        programId, 
        MyInstructions.NftMintTo,
        [
            {pubkey: authority, isSigner: true, isWritable: true},
            {pubkey: mint, isSigner: false, isWritable: true},
            {pubkey: recipientAssociatedToken, isSigner: false, isWritable: true},
            {pubkey: metadata, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

// Read

export function createReadNftInstruction(
    mint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newMetadata = getMetadataAddress(mint)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadNft,
        [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: newMetadata, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
import * as borsh from "borsh"
import { Buffer } from "buffer"
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, MyInstructions } from "."

// Create

class CreateMintInstructionData {
    instruction: MyInstructions
    decimals: number
    constructor(props: {
        instruction: MyInstructions,
        decimals: number,
    }) {
        this.instruction = props.instruction
        this.decimals = props.decimals
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateMintInstructionDataSchema, this)) 
    }
}

const CreateMintInstructionDataSchema = new Map([
    [ CreateMintInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['decimals', 'u8'],
        ],
    }]
])

function createCreateInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateMintInstructionData({instruction, decimals})

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateMint) return [
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMint, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateMintWithPayer) return [
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMint, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
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

export function createCreateMintInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, decimals, MyInstructions.CreateMint)
}

export function createCreateMintWithPayerInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
): TransactionInstruction {
    return createCreateInstruction(newMint, payer, programId, decimals, MyInstructions.CreateMintWithPayer)
}

// Mint To

class MintMintToInstructionData {
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
        return Buffer.from(borsh.serialize(MintMintToInstructionDataSchema, this)) 
    }
}

const MintMintToInstructionDataSchema = new Map([
    [ MintMintToInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

export function createMintMintToInstruction(
    mint: PublicKey,
    recipient: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
    amount: number,
): TransactionInstruction {

    const myInstructionObject = new MintMintToInstructionData({
        instruction: MyInstructions.MintMintTo, 
        amount,
    })

    const recipientAssociatedToken = getAssociatedTokenAddressSync(mint, recipient)

    const keys = [
        {pubkey: authority, isSigner: true, isWritable: true},
        {pubkey: mint, isSigner: false, isWritable: true},
        {pubkey: recipientAssociatedToken, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

// Disable Minting

export function createMintDisableMintingInstruction(
    instruction: MyInstructions,
    mint: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        instruction,
        [
            {pubkey: authority, isSigner: true, isWritable: true},
            {pubkey: mint, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

// Read

export function createReadMintInstruction(
    newMint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadMint,
        [
            {pubkey: newMint, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, MyInstructions } from "."
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token"

// Create

class CreateAssociatedTokenInstructionData {
    instruction: MyInstructions
    constructor(props: {
        instruction: MyInstructions,
    }) {
        this.instruction = props.instruction
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateAssociatedTokenInstructionDataSchema, this)) 
    }
}

const CreateAssociatedTokenInstructionDataSchema = new Map([
    [ CreateAssociatedTokenInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

function createInstruction(
    mint: PublicKey,
    owner: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateAssociatedTokenInstructionData({instruction})

    const newAssociatedToken = getAssociatedTokenAddressSync(mint, owner)

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateAssociatedToken) return [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: newAssociatedToken, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: false, isWritable: false},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateAssociatedTokenWithPayer) return [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: newAssociatedToken, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: false, isWritable: false},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
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

export function createCreateAssociatedTokenInstruction(
    mint: PublicKey,
    owner: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(mint, owner, payer, programId, MyInstructions.CreateAssociatedToken)
}

export function createCreateAssociatedTokenWithPayerInstruction(
    mint: PublicKey,
    owner: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(mint, owner, payer, programId, MyInstructions.CreateAssociatedTokenWithPayer)
}

// Burn

class BurnTokensInstructionData {
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
        return Buffer.from(borsh.serialize(BurnTokensInstructionDataSchema, this)) 
    }
}

const BurnTokensInstructionDataSchema = new Map([
    [ BurnTokensInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

export function createBurnTokensInstruction(
    mint: PublicKey,
    from: PublicKey,
    programId: PublicKey,
    amount: number,
): TransactionInstruction {

    const myInstructionObject = new BurnTokensInstructionData({
        instruction: MyInstructions.BurnTokens, 
        amount,
    })

    const fromAssociatedToken = getAssociatedTokenAddressSync(mint, from)

    const keys = [
        {pubkey: from, isSigner: true, isWritable: true},
        {pubkey: fromAssociatedToken, isSigner: false, isWritable: true},
        {pubkey: mint, isSigner: false, isWritable: true},
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

// Transfer

class TransferTokensInstructionData {
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
        return Buffer.from(borsh.serialize(TransferTokensInstructionDataSchema, this)) 
    }
}

const TransferTokensInstructionDataSchema = new Map([
    [ TransferTokensInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

export function createTransferTokensInstruction(
    mint: PublicKey,
    from: PublicKey,
    to: PublicKey,
    programId: PublicKey,
    amount: number,
): TransactionInstruction {

    const myInstructionObject = new TransferTokensInstructionData({
        instruction: MyInstructions.TransferTokens, 
        amount,
    })

    const fromAssociatedToken = getAssociatedTokenAddressSync(mint, from)
    const toAssociatedToken = getAssociatedTokenAddressSync(mint, to)

    const keys = [
        {pubkey: from, isSigner: true, isWritable: true},
        {pubkey: fromAssociatedToken, isSigner: false, isWritable: true},
        {pubkey: toAssociatedToken, isSigner: false, isWritable: true},
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

// Freeze

export function createFreezeAssociatedTokenInstruction(
    mint: PublicKey,
    owner: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const associatedToken = getAssociatedTokenAddressSync(mint, owner)
    return createBaseInstruction(
        programId, 
        MyInstructions.FreezeAccount,
        [
            {pubkey: associatedToken, isSigner: false, isWritable: true},
            {pubkey: authority, isSigner: true, isWritable: true},
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

// Thaw

export function createThawAssociatedTokenInstruction(
    mint: PublicKey,
    owner: PublicKey,
    authority: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const associatedToken = getAssociatedTokenAddressSync(mint, owner)
    return createBaseInstruction(
        programId, 
        MyInstructions.ThawAccount,
        [
            {pubkey: associatedToken, isSigner: false, isWritable: true},
            {pubkey: authority, isSigner: true, isWritable: true},
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

// Read

export function createReadAssociatedTokenInstruction(
    mint: PublicKey,
    owner: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newAssociatedToken = getAssociatedTokenAddressSync(mint, owner)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadAssociatedToken,
        [
            {pubkey: newAssociatedToken, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
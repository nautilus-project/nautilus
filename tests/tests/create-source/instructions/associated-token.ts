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

export function createCreateAssociatedTokenWithPayerInstruction(
    mint: PublicKey,
    owner: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(mint, owner, payer, programId, MyInstructions.CreateAssociatedTokenWithPayer)
}

export function createReadAssociatedTokenCreatedWithPayerInstruction(
    mint: PublicKey,
    owner: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newAssociatedToken = getAssociatedTokenAddressSync(mint, owner)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadAssociatedTokenCreatedWithPayer,
        [
            {pubkey: newAssociatedToken, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
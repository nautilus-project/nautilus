import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata'

export enum TestInstruction {
    Wallets,
    Mints,
    Metadatas,
    AssociatedTokens,
    Tokens,
}

class TestInstructionData {
    instruction: TestInstruction
    constructor(props: {
        instruction: TestInstruction,
    }) {
        this.instruction = props.instruction;
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(TestInstructionDataSchema, this)) 
    }
}

const TestInstructionDataSchema = new Map([
    [ TestInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

export function createTestInstruction(
    payer: PublicKey,
    programId: PublicKey,
    instruction: TestInstruction,
): TransactionInstruction {

    const myInstructionObject = new TestInstructionData({instruction})

    function deriveKeys(instruction: TestInstruction) {
        if (instruction === TestInstruction.Wallets) return [
            {pubkey: payer, isSigner: false, isWritable: false},    // To Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // From Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // Fee Payer
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ]
        else if (instruction === TestInstruction.Mints) return [
            {pubkey: payer, isSigner: false, isWritable: false},    // To Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // From Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // Fee Payer
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === TestInstruction.Metadatas) return [
            {pubkey: payer, isSigner: false, isWritable: false},    // To Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // From Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // Fee Payer
            {pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === TestInstruction.AssociatedTokens) return [
            {pubkey: payer, isSigner: false, isWritable: false},    // To Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // From Self Account
            {pubkey: payer, isSigner: false, isWritable: false},    // Fee Payer
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else return [
            {pubkey: payer, isSigner: false, isWritable: false},    // To Mint (Self Account)
            {pubkey: payer, isSigner: false, isWritable: false},    // To Metadata
            {pubkey: payer, isSigner: false, isWritable: false},    // From Mint (Self Account)
            {pubkey: payer, isSigner: false, isWritable: false},    // From Metadata
            {pubkey: payer, isSigner: false, isWritable: false},    // Fee Payer
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: TOKEN_METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
    }

    const keys = deriveKeys(instruction)

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}
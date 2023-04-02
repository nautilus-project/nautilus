import * as borsh from "borsh"
import { Buffer } from "buffer"
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

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

function createInstruction(
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
    return createInstruction(newMint, payer, programId, decimals, MyInstructions.CreateMint)
}

export function createCreateMintWithPayerInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
): TransactionInstruction {
    return createInstruction(newMint, payer, programId, decimals, MyInstructions.CreateMintWithPayer)
}
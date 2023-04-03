import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

class CreateWalletInstructionData {
    instruction: MyInstructions
    constructor(props: {
        instruction: MyInstructions,
    }) {
        this.instruction = props.instruction
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateWalletInstructionDataSchema, this)) 
    }
}

const CreateWalletInstructionDataSchema = new Map([
    [ CreateWalletInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

function createInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateWalletInstructionData({instruction})

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateWallet) return [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateWalletWithPayer) return [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
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

export function createCreateWalletInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(newWallet, payer, programId, MyInstructions.CreateWallet)
}

export function createCreateWalletWithPayerInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(newWallet, payer, programId, MyInstructions.CreateWalletWithPayer)
}
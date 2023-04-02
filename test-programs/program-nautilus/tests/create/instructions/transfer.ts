import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

class TransferInstructionData {
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
        return Buffer.from(borsh.serialize(TransferInstructionDataSchema, this)) 
    }
}

const TransferInstructionDataSchema = new Map([
    [ TransferInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

function createInstruction(
    from: PublicKey,
    to: PublicKey,
    programId: PublicKey,
    amount: number,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new TransferInstructionData({instruction, amount})

    const keys = [
        {pubkey: from, isSigner: true, isWritable: true},
        {pubkey: to, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createTransferWalletInstruction(
    from: PublicKey,
    to: PublicKey,
    programId: PublicKey,
    amount: number,
): TransactionInstruction {
    return createInstruction(from, to, programId, amount, MyInstructions.TransferWallet)
}

import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SYSVAR_RENT_PUBKEY, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

class ComplexInstructionData {
    instruction: MyInstructions
    amount_to_fund: number
    amount_to_transfer: number
    constructor(props: {
        instruction: MyInstructions,
        amount_to_fund: number,
        amount_to_transfer: number,
    }) {
        this.instruction = props.instruction
        this.amount_to_fund = props.amount_to_fund
        this.amount_to_transfer = props.amount_to_transfer
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(ComplexInstructionDataSchema, this)) 
    }
}

const ComplexInstructionDataSchema = new Map([
    [ ComplexInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount_to_fund', 'u64'],
            ['amount_to_transfer', 'u64'],
        ],
    }]
])

function createInstruction(
    authority1: PublicKey,
    authority2: PublicKey,
    rentPayer1: PublicKey,
    rentPayer2: PublicKey,
    transferRecipient: PublicKey,
    walletAllocate: PublicKey,
    walletCreate: PublicKey,
    walletCreateWithTransferSafe: PublicKey,
    walletCreateWithTransferUnsafe: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    amount_to_fund: number,
    amount_to_transfer: number,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new ComplexInstructionData({instruction, amount_to_fund, amount_to_transfer})

    const keys = [
        {pubkey: authority1, isSigner: true, isWritable: true},
        {pubkey: authority2, isSigner: true, isWritable: true},
        {pubkey: rentPayer1, isSigner: true, isWritable: true},
        {pubkey: rentPayer2, isSigner: true, isWritable: true},
        {pubkey: transferRecipient, isSigner: false, isWritable: true},
        {pubkey: walletAllocate, isSigner: true, isWritable: true},
        {pubkey: walletCreate, isSigner: true, isWritable: true},
        {pubkey: walletCreateWithTransferSafe, isSigner: false, isWritable: true},
        {pubkey: walletCreateWithTransferUnsafe, isSigner: false, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createComplexInstruction(
    authority1: PublicKey,
    authority2: PublicKey,
    rentPayer1: PublicKey,
    rentPayer2: PublicKey,
    transferRecipient: PublicKey,
    walletAllocate: PublicKey,
    walletCreate: PublicKey,
    walletCreateWithTransferSafe: PublicKey,
    walletCreateWithTransferUnsafe: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    amountToFund: number,
    amountToTransfer: number,
): TransactionInstruction {
    return createInstruction(
        authority1,
        authority2,
        rentPayer1,
        rentPayer2,
        transferRecipient,
        walletAllocate,
        walletCreate,
        walletCreateWithTransferSafe,
        walletCreateWithTransferUnsafe,
        payer,
        programId,
        amountToFund,
        amountToTransfer,
        MyInstructions.Complex,
    )
}

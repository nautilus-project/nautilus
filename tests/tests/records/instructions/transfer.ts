import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions, deriveCarAddress, deriveHomeAddress, derivePersonAddress, fetchIndex } from "."
import assert from "assert"

class FundOrTransferInstructionData {
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
        return Buffer.from(borsh.serialize(FundOrTransferInstructionDataSchema, this)) 
    }
}

const FundOrTransferInstructionDataSchema = new Map([
    [ FundOrTransferInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['amount', 'u64'],
        ],
    }]
])

function createInstruction(
    programId: PublicKey,
    amount: number,
    instruction: MyInstructions,
    keys: {pubkey: PublicKey, isSigner: boolean, isWritable: boolean}[]
): TransactionInstruction {

    const myInstructionObject = new FundOrTransferInstructionData({instruction, amount})

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export async function createFundPersonInstruction(
    payer: PublicKey,
    programId: PublicKey,
    amount: number,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("person");
    assert(currentId != undefined)
    const record = derivePersonAddress(programId, currentId + 1)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.FundPerson, keys)
}

export async function createTransferFromPersonInstruction(
    recipient: PublicKey,
    programId: PublicKey,
    amount: number,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("person");
    assert(currentId != undefined)
    const record = derivePersonAddress(programId, currentId + 1)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: recipient, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.TransferFromPerson, keys)
}

export async function createFundHomeInstruction(
    payer: PublicKey,
    programId: PublicKey,
    amount: number,
    homeId: number
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const record = deriveHomeAddress(programId, homeId)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.FundHome, keys)
}

export async function createTransferFromHomeInstruction(
    recipient: PublicKey,
    programId: PublicKey,
    amount: number,
    homeId: number,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const record = deriveHomeAddress(programId, homeId)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: recipient, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.TransferFromHome, keys)
}

export async function createFundCarInstruction(
    payer: PublicKey,
    programId: PublicKey,
    amount: number,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("car");
    assert(currentId != undefined)
    const record = deriveCarAddress(programId, currentId + 1)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.FundCar, keys)
}

export async function createTransferFromCarInstruction(
    recipient: PublicKey,
    programId: PublicKey,
    amount: number,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("car");
    assert(currentId != undefined)
    const record = deriveCarAddress(programId, currentId + 1)
    const keys = [
        {pubkey: index[0], isSigner: false, isWritable: true},
        {pubkey: record, isSigner: false, isWritable: true},
        {pubkey: recipient, isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]
    return createInstruction(programId, amount, MyInstructions.TransferFromCar, keys)
}
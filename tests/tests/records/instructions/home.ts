import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, deriveIndexAddress, fetchIndex, MyInstructions } from "."

class CreateHomeInstructionData {
    instruction: MyInstructions
    id: number
    house_number: number
    street: string
    constructor(props: {
        instruction: MyInstructions,
        id: number,
        house_number: number,
        street: string,
    }) {
        this.instruction = props.instruction
        this.id = props.id
        this.house_number = props.house_number
        this.street = props.street
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateHomeInstructionDataSchema, this)) 
    }
}

const CreateHomeInstructionDataSchema = new Map([
    [ CreateHomeInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['id', 'u8'],
            ['house_number', 'u8'],
            ['street', 'string'],
        ],
    }]
])

export function deriveHomeAddress(programId: PublicKey, id: number): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("home"), Buffer.from(Uint8Array.of(id))],
        programId
    )[0]
}

function createInstruction(
    index: PublicKey,
    newRecord: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    id: number,
    house_number: number,
    street: string,
): TransactionInstruction {

    const myInstructionObject = new CreateHomeInstructionData({
        instruction: MyInstructions.CreateHome, 
        id,
        house_number,
        street,
    })

    const keys = [
        {pubkey: index, isSigner: false, isWritable: true},
        {pubkey: newRecord, isSigner: false, isWritable: true},
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

export function createCreateHomeInstruction(
    payer: PublicKey,
    programId: PublicKey,
    id: number,
    house_number: number,
    street: string,
): TransactionInstruction {
    const index = deriveIndexAddress(programId)
    const newRecord = deriveHomeAddress(programId, id)
    return createInstruction(index, newRecord, payer, programId, id, house_number, street)
}

export function createReadHomeInstruction(
    programId: PublicKey,
    id: number,
): TransactionInstruction {
    const index = deriveIndexAddress(programId)
    const record = deriveHomeAddress(programId, id)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadHome,
        [
            {pubkey: index, isSigner: false, isWritable: false},
            {pubkey: record, isSigner: false, isWritable: false},
        ],
    )
}
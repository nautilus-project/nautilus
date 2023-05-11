import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, fetchIndex, MyInstructions } from "."
import assert from "assert"

class CreateCarInstructionData {
    instruction: MyInstructions
    make: string
    model: string
    purchase_authority: Uint8Array
    operating_authority: Uint8Array
    constructor(props: {
        instruction: MyInstructions,
        make: string,
        model: string,
        purchase_authority: PublicKey,
        operating_authority: PublicKey,
    }) {
        this.instruction = props.instruction
        this.make = props.make
        this.model = props.model
        this.purchase_authority = props.purchase_authority.toBuffer()
        this.operating_authority = props.operating_authority.toBuffer()
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateCarInstructionDataSchema, this)) 
    }
}

const CreateCarInstructionDataSchema = new Map([
    [ CreateCarInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['make', 'string'],
            ['model', 'string'],
            ['purchase_authority', [32]],
            ['operating_authority', [32]],
        ],
    }]
])

function deriveCarAddress(programId: PublicKey, id: number): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("car"), Buffer.from(Uint8Array.of(id))],
        programId
    )[0]
}

function createInstruction(
    index: PublicKey,
    newRecord: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    make: string,
    model: string,
    purchase_authority: PublicKey,
    operating_authority: PublicKey,
): TransactionInstruction {

    const myInstructionObject = new CreateCarInstructionData({
        instruction: MyInstructions.CreateCar, 
        make,
        model,
        purchase_authority,
        operating_authority,
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

export async function createCreateCarInstruction(
    payer: PublicKey,
    programId: PublicKey,
    make: string,
    model: string,
    purchase_authority: PublicKey,
    operating_authority: PublicKey,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("car");
    assert(currentId != undefined)
    const newRecord = deriveCarAddress(programId, currentId + 1)
    return createInstruction(index[0], newRecord, payer, programId, make, model, purchase_authority, operating_authority)
}

export async function createReadCarInstruction(
    programId: PublicKey,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("car");
    assert(currentId != undefined)
    const record = deriveCarAddress(programId, currentId + 1) // TODO
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadCar,
        [
            {pubkey: index[0], isSigner: false, isWritable: false},
            {pubkey: record, isSigner: false, isWritable: false},
        ],
    )
}
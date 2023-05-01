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

class CreatePersonInstructionData {
    instruction: MyInstructions
    name: string
    authority: Uint8Array
    constructor(props: {
        instruction: MyInstructions,
        name: string,
        authority: PublicKey,
    }) {
        this.instruction = props.instruction
        this.name = props.name
        this.authority = props.authority.toBuffer()
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreatePersonInstructionDataSchema, this)) 
    }
}

const CreatePersonInstructionDataSchema = new Map([
    [ CreatePersonInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['name', 'string'],
            ['authority', [32]],
        ],
    }]
])

function derivePersonAddress(programId: PublicKey, id: number): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("person"), Buffer.from(Uint8Array.of(id))],
        programId
    )[0]
}

function createInstruction(
    index: PublicKey,
    newRecord: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    name: string,
    authority: PublicKey,
): TransactionInstruction {

    const myInstructionObject = new CreatePersonInstructionData({
        instruction: MyInstructions.CreatePerson, 
        name,
        authority,
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

export async function createCreatePersonInstruction(
    payer: PublicKey,
    programId: PublicKey,
    name: string,
    authority: PublicKey,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("person");
    assert(currentId != undefined)
    const newRecord = derivePersonAddress(programId, currentId + 1)
    return createInstruction(index[0], newRecord, payer, programId, name, authority)
}

export async function createReadPersonInstruction(
    programId: PublicKey,
): Promise<TransactionInstruction> {
    const index = await fetchIndex(programId)
    const currentId = index[1].get("person");
    assert(currentId != undefined)
    const record = derivePersonAddress(programId, currentId + 1) // TODO
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadPerson,
        [
            {pubkey: index[0], isSigner: false, isWritable: false},
            {pubkey: record, isSigner: false, isWritable: false},
        ],
    )
}
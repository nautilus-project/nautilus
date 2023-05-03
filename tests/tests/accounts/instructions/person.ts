import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, MyInstructions } from "."
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

function derivePersonAddress(programId: PublicKey, authority: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("person"), authority.toBuffer()],
        programId
    )[0]
}

function createInstruction(
    newAccount: PublicKey,
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
        {pubkey: newAccount, isSigner: false, isWritable: true},
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
    const newAccount = derivePersonAddress(programId, authority)
    return createInstruction(newAccount, payer, programId, name, authority)
}

export async function createReadPersonInstruction(
    programId: PublicKey,
    authority: PublicKey,
): Promise<TransactionInstruction> {
    const account = derivePersonAddress(programId, authority) // TODO
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadPerson,
        [
            {pubkey: account, isSigner: false, isWritable: false},
        ],
    )
}
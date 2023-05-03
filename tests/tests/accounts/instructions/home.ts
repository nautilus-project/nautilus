import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { createBaseInstruction, MyInstructions } from "."

class CreateHomeInstructionData {
    instruction: MyInstructions
    house_number: number
    street: string
    some_pubkey: Uint8Array
    constructor(props: {
        instruction: MyInstructions,
        house_number: number,
        street: string,
        some_pubkey: PublicKey,
    }) {
        this.instruction = props.instruction
        this.house_number = props.house_number
        this.street = props.street
        this.some_pubkey = props.some_pubkey.toBuffer()
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
            ['house_number', 'u8'],
            ['street', 'string'],
            ['some_pubkey', [32]],
        ],
    }]
])

function deriveHomeAddress(programId: PublicKey, somePubkey: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("home"), somePubkey.toBuffer()],
        programId
    )[0]
}

function createInstruction(
    newAccount: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    house_number: number,
    street: string,
    some_pubkey: PublicKey,
): TransactionInstruction {

    const myInstructionObject = new CreateHomeInstructionData({
        instruction: MyInstructions.CreateHome, 
        house_number,
        street,
        some_pubkey,
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

export function createCreateHomeInstruction(
    payer: PublicKey,
    programId: PublicKey,
    house_number: number,
    street: string,
    somePubkey: PublicKey,
): TransactionInstruction {
    const newAccount = deriveHomeAddress(programId, somePubkey)
    return createInstruction(newAccount, payer, programId, house_number, street, somePubkey)
}

export function createReadHomeInstruction(
    programId: PublicKey,
    somePubkey: PublicKey,
): TransactionInstruction {
    const account = deriveHomeAddress(programId, somePubkey)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadHome,
        [
            {pubkey: account, isSigner: false, isWritable: false},
        ],
    )
}
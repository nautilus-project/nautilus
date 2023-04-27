export * from './car'
export * from './home'
export * from './person'
export * from './token'

import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction } from '@solana/web3.js'
import * as borsh from "borsh"
import { Buffer } from "buffer"
import { TEST_CONFIGS } from '../../const'

export enum MyInstructions {
    CreateToken,
    ReadToken,
    Initialize,
    CreatePerson,
    ReadPerson,
    CreateHome,
    ReadHome,
    CreateCar,
    ReadCar,
}

export class BaseInstructionData {
    instruction: MyInstructions
    constructor(props: {
        instruction: MyInstructions,
    }) {
        this.instruction = props.instruction
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(BaseInstructionDataSchema, this)) 
    }
}

const BaseInstructionDataSchema = new Map([
    [ BaseInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

export function createBaseInstruction(
    programId: PublicKey,
    instruction: MyInstructions,
    keys: {pubkey: PublicKey, isSigner: boolean, isWritable: boolean}[],
): TransactionInstruction {

    const myInstructionObject = new BaseInstructionData({instruction})
    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function deriveIndexAddress(programId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("nautilus_index"), Buffer.from(Uint8Array.of(0))],
        programId
    )[0]
}

export async function fetchIndex(programId: PublicKey): Promise<[PublicKey, Map<string, number>]> {
    const connection = TEST_CONFIGS.connection
    const indexPubkey = deriveIndexAddress(programId)
    const indexAccountInfo = await connection.getAccountInfo(indexPubkey)
    return [indexPubkey ,new Map<string, number>([
        ["person", 0],
        ["home", 0],
        ["car", 0],
    ])] // TODO
}

export function createInitializeInstruction(payer: PublicKey, programId: PublicKey): TransactionInstruction {
    const indexPubkey = deriveIndexAddress(programId)
    return createBaseInstruction(
        programId,
        MyInstructions.Initialize,
        [
            {pubkey: indexPubkey, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
    )
}
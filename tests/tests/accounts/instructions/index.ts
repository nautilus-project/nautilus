export * from './car'
export * from './home'
export * from './person'

import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction } from '@solana/web3.js'
import * as borsh from "borsh"
import { Buffer } from "buffer"
import { TEST_CONFIGS } from '../../const'

export enum MyInstructions {
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
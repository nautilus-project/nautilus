import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'

export enum TestInstruction {
    One,
    Two,
}

class TestData {
    instruction: TestInstruction
    constructor(props: {
        instruction: TestInstruction,
    }) {
        this.instruction = props.instruction;
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(TestDataSchema, this)) 
    }
}

const TestDataSchema = new Map([
    [ TestData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

export function createTestInstruction(
    payer: PublicKey,
    programId: PublicKey,
    instruction: TestInstruction,
): TransactionInstruction {

    const myInstructionObject = new TestData({instruction})

    const keys = [
        {pubkey: payer, isSigner: false, isWritable: false},
        {pubkey: payer, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

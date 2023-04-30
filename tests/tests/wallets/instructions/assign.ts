import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

class AssignInstructionData {
    instruction: MyInstructions
    owner: Uint8Array
    constructor(props: {
        instruction: MyInstructions,
        owner: PublicKey,
    }) {
        this.instruction = props.instruction
        this.owner = props.owner.toBuffer()
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(AssignInstructionDataSchema, this)) 
    }
}

const AssignInstructionDataSchema = new Map([
    [ AssignInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['owner', [32]],
        ],
    }]
])

function createInstruction(
    wallet: PublicKey,
    programId: PublicKey,
    owner: PublicKey,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new AssignInstructionData({instruction, owner})

    const keys = [
        {pubkey: wallet, isSigner: true, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createAssignInstruction(
    wallet: PublicKey,
    programId: PublicKey,
    owner: PublicKey,
): TransactionInstruction {
    return createInstruction(wallet, programId, owner, MyInstructions.Assign)
}

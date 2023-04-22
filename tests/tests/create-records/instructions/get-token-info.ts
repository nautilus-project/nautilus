import * as borsh from "borsh"
import { Buffer } from "buffer"
import { PROGRAM_ID as METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata'
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions } from "."

class GetTokenInfoInstructionData {
    instruction: MyInstructions
    constructor(props: {
        instruction: MyInstructions,
    }) {
        this.instruction = props.instruction
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(GetTokenInfoInstructionDataSchema, this)) 
    }
}

const GetTokenInfoInstructionDataSchema = new Map([
    [ GetTokenInfoInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

function createInstruction(
    mint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new GetTokenInfoInstructionData({instruction})

    const metadata = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
        METADATA_PROGRAM_ID,
    )[0]

    const keys = [
        {pubkey: mint, isSigner: false, isWritable: false},
        {pubkey: metadata, isSigner: false, isWritable: false},
        {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createGetTokenInfoInstruction(
    mint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createInstruction(mint, payer, programId, MyInstructions.GetTokenInfo)
}
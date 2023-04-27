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
import { createBaseInstruction, MyInstructions } from "."

class CreateTokenInstructionData {
    instruction: MyInstructions
    decimals: number
    title: string
    symbol: string
    uri: string
    constructor(props: {
        instruction: MyInstructions,
        decimals: number
        title: string,
        symbol: string,
        uri: string,
    }) {
        this.instruction = props.instruction
        this.decimals = props.decimals
        this.title = props.title
        this.symbol = props.symbol
        this.uri = props.uri
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateTokenInstructionDataSchema, this)) 
    }
}

const CreateTokenInstructionDataSchema = new Map([
    [ CreateTokenInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['decimals', 'u8'],
            ['title', 'string'],
            ['symbol', 'string'],
            ['uri', 'string'],
        ],
    }]
])

function getMetadataAddress(mint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
        METADATA_PROGRAM_ID,
    )[0]
}

function createInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {

    const myInstructionObject = new CreateTokenInstructionData({
        instruction: MyInstructions.CreateToken, 
        decimals, 
        title, 
        symbol, 
        uri
    })

    const newMetadata = getMetadataAddress(newMint)

    const keys = [
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: newMint, isSigner: true, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: newMetadata, isSigner: false, isWritable: true},
        {pubkey: payer, isSigner: true, isWritable: true},
        {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
    ]

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createCreateTokenInstruction(
    newMint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    decimals: number,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createInstruction(newMint, payer, programId, decimals, title, symbol, uri)
}

export function createReadTokenInstruction(
    mint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newMetadata = getMetadataAddress(mint)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadToken,
        [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: newMetadata, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
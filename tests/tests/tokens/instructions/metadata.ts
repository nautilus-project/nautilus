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

class CreateMetadataInstructionData {
    instruction: MyInstructions
    title: string
    symbol: string
    uri: string
    constructor(props: {
        instruction: MyInstructions,
        title: string,
        symbol: string,
        uri: string,
    }) {
        this.instruction = props.instruction
        this.title = props.title
        this.symbol = props.symbol
        this.uri = props.uri
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateMetadataInstructionDataSchema, this)) 
    }
}

const CreateMetadataInstructionDataSchema = new Map([
    [ CreateMetadataInstructionData, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
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
    mint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    title: string,
    symbol: string,
    uri: string,
    instruction: MyInstructions,
): TransactionInstruction {

    const myInstructionObject = new CreateMetadataInstructionData({instruction, title, symbol, uri})

    const newMetadata = getMetadataAddress(mint)

    function deriveKeys(instruction: MyInstructions) {
        if (instruction === MyInstructions.CreateMetadata) return [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMetadata, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        else if (instruction === MyInstructions.CreateMetadataWithPayer) return [
            {pubkey: mint, isSigner: false, isWritable: false},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: newMetadata, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ]
        return []
    }

    const keys = deriveKeys(instruction)

    return new TransactionInstruction({
        keys,
        programId,
        data: myInstructionObject.toBuffer(),
    })
}

export function createCreateMetadataInstruction(
    mint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createInstruction(mint, payer, programId, title, symbol, uri, MyInstructions.CreateMetadata)
}

export function createReadMetadataInstruction(
    mint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newMetadata = getMetadataAddress(mint)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadMetadata,
        [
            {pubkey: newMetadata, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}

export function createCreateMetadataWithPayerInstruction(
    mint: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
    title: string,
    symbol: string,
    uri: string,
): TransactionInstruction {
    return createInstruction(mint, payer, programId, title, symbol, uri, MyInstructions.CreateMetadataWithPayer)
}

export function createReadMetadataCreatedWithPayerInstruction(
    mint: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    const newMetadata = getMetadataAddress(mint)
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadMetadataCreatedWithPayer,
        [
            {pubkey: newMetadata, isSigner: false, isWritable: false},
            {pubkey: METADATA_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
    )
}
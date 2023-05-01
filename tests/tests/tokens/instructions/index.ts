export * from './associated-token'
export * from './mint'
export * from './metadata'
export * from './nft'
export * from './token'

import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js'
import * as borsh from "borsh"
import { Buffer } from "buffer"

export enum MyInstructions {
    CreateMint,
    CreateMintWithPayer,
    MintMintTo,
    MintDisableMinting,
    ReadMint,
    CreateMetadata,
    CreateMetadataWithPayer,
    ReadMetadata,
    CreateAssociatedToken,
    CreateAssociatedTokenWithPayer,
    ReadAssociatedToken,
    BurnTokens,
    TransferTokens,
    FreezeAccount,
    ThawAccount,
    CreateToken,
    CreateTokenWithPayer,
    TokenMintTo,
    TokenDisableMinting,
    ReadToken,
    CreateNft,
    CreateNftWithPayer,
    NftMintTo,
    ReadNft,
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
export * from './associated-token'
export * from './mint'
export * from './metadata'
export * from './token'
export * from './transfer'
export * from './wallet'

import { PublicKey, TransactionInstruction } from '@solana/web3.js'
import * as borsh from "borsh"
import { Buffer } from "buffer"

export enum MyInstructions {
    CreateWallet,
    ReadWallet,
    CreateWalletWithPayer,
    ReadWalletCreatedWithPayer,
    CreateMint,
    ReadMint,
    CreateMintWithPayer,
    ReadMintCreatedWithPayer,
    CreateMetadata,
    ReadMetadata,
    CreateMetadataWithPayer,
    ReadMetadataCreatedWithPayer,
    CreateAssociatedToken,
    ReadAssociatedToken,
    CreateAssociatedTokenWithPayer,
    ReadAssociatedTokenCreatedWithPayer,
    CreateToken,
    ReadToken,
    CreateTokenWithPayer,
    ReadTokenCreatedWithPayer,
    TransferWallet,
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
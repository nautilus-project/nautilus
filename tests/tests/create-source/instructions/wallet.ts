import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions, createBaseInstruction } from "."

export function createCreateWalletInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.CreateWallet,
        [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
    )
}

export function createReadWalletInstruction(
    newWallet: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadWallet,
        [
            {pubkey: newWallet, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
    )
}

export function createCreateWalletWithPayerInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.CreateWalletWithPayer,
        [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ]
    )
}

export function createReadWalletCreatedWithPayerInstruction(
    newWallet: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.ReadWalletCreatedWithPayer,
        [
            {pubkey: newWallet, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
    )
}
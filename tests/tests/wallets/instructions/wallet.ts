import { 
    PublicKey, 
    SystemProgram, 
    SYSVAR_RENT_PUBKEY, 
    TransactionInstruction 
} from '@solana/web3.js'
import { MyInstructions, createBaseInstruction } from "."

export function createAllocateWalletInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.Allocate,
        [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
    )
}

export function createCreateWalletInstruction(
    newWallet: PublicKey,
    payer: PublicKey,
    programId: PublicKey,
): TransactionInstruction {
    return createBaseInstruction(
        programId, 
        MyInstructions.Create,
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
        MyInstructions.Read,
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
        MyInstructions.CreateWithPayer,
        [
            {pubkey: newWallet, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ]
    )
}
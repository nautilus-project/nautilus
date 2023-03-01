import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    SystemProgram, 
    TransactionInstruction 
} from '@solana/web3.js'

enum MyInstruction {
    CreateHero,
    DeleteHero,
    UpdateHero,
}

class CreateHero {
    instruction: MyInstruction
    id: number
    name: string
    authority: Uint8Array
    constructor(props: {
        instruction: MyInstruction,
        id: number,
        name: string,
        authority: PublicKey,
    }) {
        this.instruction = props.instruction;
        this.id = props.id;
        this.name = props.name;
        this.authority = props.authority.toBytes();
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateHeroSchema, this)) 
    }
}

const CreateHeroSchema = new Map([
    [ CreateHero, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['id', 'u8'],
            ['name', 'string'],
            ['authority', [32]],
        ],
    }]
])

export function createCreateHeroInstruction(
    payer: PublicKey,
    programId: PublicKey,
    id: number,
    name: string,
    authority: PublicKey,
): TransactionInstruction {

    const autoincrementPublicKey = PublicKey.findProgramAddressSync(
        [Buffer.from("hero_autoincrement")],
        programId,
    )[0];

    const newAccountPublicKey = PublicKey.findProgramAddressSync(
        [Buffer.from("hero"), Buffer.from(Uint8Array.of(id))],
        programId,
    )[0];

    const myInstructionObject = new CreateHero({
        instruction: MyInstruction.CreateHero,
        id,
        name,
        authority,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: autoincrementPublicKey, isSigner: false, isWritable: true},
            {pubkey: newAccountPublicKey, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: false, isWritable: false}, // Authority
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}

class UpdateHero {
    instruction: MyInstruction
    id: number
    name: string
    authority: Uint8Array
    constructor(props: {
        instruction: MyInstruction,
        id: number,
        name: string,
        authority: PublicKey,
    }) {
        this.instruction = props.instruction;
        this.id = props.id;
        this.name = props.name;
        this.authority = props.authority.toBytes();
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(UpdateHeroSchema, this)) 
    }
}

const UpdateHeroSchema = new Map([
    [ UpdateHero, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
            ['id', 'u8'],
            ['name', 'string'],
            ['authority', [32]],
        ],
    }]
])

export function createUpdateHeroInstruction(
    payer: PublicKey,
    programId: PublicKey,
    id: number,
    name: string,
    authority: PublicKey,
): TransactionInstruction {

    const targetAccountPublicKey = PublicKey.findProgramAddressSync(
        [Buffer.from("hero"), Buffer.from(Uint8Array.of(id))],
        programId,
    )[0];

    const myInstructionObject = new UpdateHero({
        instruction: MyInstruction.UpdateHero,
        id,
        name,
        authority,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: targetAccountPublicKey, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: false}, // Authority
            {pubkey: payer, isSigner: true, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}

class DeleteHero {
    instruction: MyInstruction
    constructor(props: {
        instruction: MyInstruction,
    }) {
        this.instruction = props.instruction;
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(DeleteHeroSchema, this)) 
    }
}

const DeleteHeroSchema = new Map([
    [ DeleteHero, { 
        kind: 'struct', 
        fields: [ 
            ['instruction', 'u8'],
        ],
    }]
])

export function createDeleteHeroInstruction(
    payer: PublicKey,
    programId: PublicKey,
    id: number,
): TransactionInstruction {

    const targetAccountPublicKey = PublicKey.findProgramAddressSync(
        [Buffer.from("hero"), Buffer.from(Uint8Array.of(id))],
        programId,
    )[0];

    const myInstructionObject = new DeleteHero({
        instruction: MyInstruction.DeleteHero,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: targetAccountPublicKey, isSigner: false, isWritable: true},
            {pubkey: payer, isSigner: true, isWritable: false}, // Authority
            {pubkey: payer, isSigner: true, isWritable: true},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}
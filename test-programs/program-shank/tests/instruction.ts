import * as borsh from "borsh"
import { Buffer } from "buffer"
import { 
    PublicKey, 
    TransactionInstruction 
} from '@solana/web3.js'

export enum MyInstruction {
    CreateHero,
    UpdateHero,
    DeleteHero,
}

export class CreateHero {
    instruction: MyInstruction
    id: number
    name: string
    authority: PublicKey
    constructor(props: {
        instruction: MyInstruction,
        id: number,
        name: string,
        authority: PublicKey,
    }) {
        this.instruction = props.instruction;
        this.id = props.id;
        this.name = props.name;
        this.authority = props.authority;
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(CreateHeroSchema, this)) 
    }
}

export const CreateHeroSchema = new Map([
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

    const myInstructionObject = new CreateHero({
        instruction: MyInstruction.CreateHero,
        id,
        name,
        authority,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: payer, isSigner: true, isWritable: true},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}

export class UpdateHero {
    instruction: MyInstruction
    id: number
    name: string
    authority: PublicKey
    constructor(props: {
        instruction: MyInstruction,
        id: number,
        name: string,
        authority: PublicKey,
    }) {
        this.instruction = props.instruction;
        this.id = props.id;
        this.name = props.name;
        this.authority = props.authority;
    }
    toBuffer() { 
        return Buffer.from(borsh.serialize(UpdateHeroSchema, this)) 
    }
}

export const UpdateHeroSchema = new Map([
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

    const myInstructionObject = new UpdateHero({
        instruction: MyInstruction.UpdateHero,
        id,
        name,
        authority,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: payer, isSigner: true, isWritable: true},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}

export class DeleteHero {
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

export const DeleteHeroSchema = new Map([
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
): TransactionInstruction {

    const myInstructionObject = new DeleteHero({
        instruction: MyInstruction.DeleteHero,
    })

    return new TransactionInstruction({
        keys: [
            {pubkey: payer, isSigner: true, isWritable: true},
        ],
        programId: programId,
        data: myInstructionObject.toBuffer(),
    })
}
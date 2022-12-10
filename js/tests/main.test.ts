import * as borsh from 'borsh';
import fs from 'fs';
import os from 'os';
import { 
    describe, 
    it 
} from 'mocha';
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { getAccounts } from '../src/main'

export function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(fs.readFileSync(path, "utf-8")))
    )
}

const TEST_PROGRAM_ID = new PublicKey('9kYnTzxTSTtKJjBBScH2m3SLBq8grogLhwMLZdcD2wG4');

class CreateEventArgs {
    id: string;
    constructor(props: {
        id: string,
    }) {
        this.id = props.id;
    }
    toBuffer() {
        return Buffer.from(borsh.serialize(CreateEventArgsSchema, this));
    }
};

const CreateEventArgsSchema = new Map([
    [
        CreateEventArgs, {
            kind: 'struct',
            fields: [
                ['id', 'string'],
            ]
        }
    ]
]);

describe("Unit Tests", async () => {

    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const payer = createKeypairFromFile(os.homedir() + '/.config/solana/id-prestige1.json');

    it("Create", async () => {
        const instructionData = new CreateEventArgs({ id: 'j' });
        const event = PublicKey.findProgramAddressSync(
            [Buffer.from('event'), Buffer.from(instructionData.id)],
            TEST_PROGRAM_ID
        )[0];
        await sendAndConfirmTransaction(
            connection,
            new Transaction().add(new TransactionInstruction({
                programId: TEST_PROGRAM_ID,
                keys: [
                    {pubkey: event, isSigner: false, isWritable: true},
                    {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                    {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
                ],
                data: instructionData.toBuffer(),
            })),
            [payer],
            { skipPreflight: true }
        )
    });

    it("Read", async () => {
        (await getAccounts(connection, TEST_PROGRAM_ID, payer.publicKey)).forEach(acc =>
            console.log(acc.account.data)
        )
    });
});
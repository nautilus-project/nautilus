//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//

import {
    Connection,
    Keypair,
    PublicKey
} from '@solana/web3.js';
import { NautilusIdl, NautilusProgram } from './types';
import {
    NautilusQuery,
    NautilusTable,
} from './sql';

import { decapitalizeFirstLetter } from './util';

export class Nautilus<Program extends NautilusProgram = NautilusProgram> {

    connection: Connection;
    programId: PublicKey;
    programs?: {[programName: string]: PublicKey};
    payer?: Keypair | undefined;

    readonly idl: NautilusIdl;
    readonly tables: { [N in keyof Program["tables"]]: NautilusTable };

    constructor ({ connection, idl, programId, programs, payer }: {
        connection: Connection,
        idl: NautilusIdl,
        programId: string | PublicKey,
        programs?: {[programName: string]: PublicKey},
        payer?: Keypair,
    }) {

        this.connection = connection;
        this.programId = programId instanceof PublicKey ? programId : new PublicKey(programId);
        this.programs = programs
        this.payer = payer ?? undefined;

        this.idl = idl

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const tables: any = {}
        for (const table of idl.accounts) {
            const formattedName = decapitalizeFirstLetter(table.name)
            tables[formattedName] = new NautilusTable(this, table.name)
        }
        this.tables = tables
    }

    table(tableName: string): NautilusTable {
        return new NautilusTable(
            this,
            tableName,
        );
    }

    sql(query: string): NautilusQuery {
        return new NautilusQuery(
            this,
            query,
        )
    }
}

export { NautilusQuery, NautilusTable }
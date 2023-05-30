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
import { NautilusIdl, NautilusTableIdl } from './idl';
import {
  NautilusQuery,
  NautilusTable,
} from './sql';

import { ProgramTables } from './types';
import { decapitalizeFirstLetter } from './util';

export class NautilusProgram<Program extends NautilusIdl = NautilusIdl> {

  connection: Connection;
  programId: PublicKey
  payer?: Keypair;

  readonly tables: ProgramTables<Program>;

  constructor ({ connection, idl, programId, payer }: {
    connection: Connection,
    idl: NautilusIdl,
    programId: PublicKey,
    payer?: Keypair,
  }) {
    this.connection = connection;
    this.payer = payer ?? undefined;
    this.programId = programId

    const tables: any = {}

    for (const table of (idl.accounts || [])) {
      if (table.config && table.config.tableName) {
        const formattedName = decapitalizeFirstLetter(table.name)
        tables[formattedName] = new NautilusTable(this, table as NautilusTableIdl)
      }
    }

    this.tables = tables
  }

  sql(query: string): NautilusQuery<[Program]> {
    return new NautilusQuery(
      [this],
      query,
    )
  }
}

export { NautilusQuery, NautilusTable }

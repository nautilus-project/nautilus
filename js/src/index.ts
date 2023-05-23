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
import {
  NautilusQuery,
  NautilusTable,
} from './sql';

import { NautilusIdl } from './idl';
import { ProgramsTables } from './types';
import { decapitalizeFirstLetter } from './util';

export class Nautilus<Programs extends NautilusIdl[] = NautilusIdl[]> {

  connection: Connection;
  programId: PublicKey;
  programs?: { [programName: string]: PublicKey };
  payer?: Keypair | undefined;

  readonly tables: ProgramsTables<Programs>;

  constructor ({ connection, idl, programId, programs, payer }: {
    connection: Connection,
    idl: NautilusIdl,
    programId: string | PublicKey,
    programs?: { [programName: string]: PublicKey },
    payer?: Keypair,
  }) {
    this.connection = connection;
    this.programId = programId instanceof PublicKey ? programId : new PublicKey(programId);
    this.programs = programs
    this.payer = payer ?? undefined;

    const tables: any = {}
    for (const table of idl.accounts) {
      if (table.config && table.config.tableName) {
        const formattedName = decapitalizeFirstLetter(table.name)
        tables[formattedName] = new NautilusTable(this, table.name)
      }
    }
    this.tables = tables
  }

  getTable(tableName: string): NautilusTable {
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

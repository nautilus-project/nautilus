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
import { InputPrograms, ProgramsTables } from './types';
import {
  NautilusQuery,
  NautilusTable,
} from './sql';

import { NautilusIdl } from './idl';
import { decapitalizeFirstLetter } from './util';

export class Nautilus<Programs extends NautilusIdl[] = NautilusIdl[]> {

  connection: Connection;
  // Lookups program key using its name
  programs: { [Program in Programs[number]as Program["name"]]: PublicKey };
  payer?: Keypair;

  readonly tables: ProgramsTables<Programs>;

  constructor ({ connection, inputPrograms, payer }: {
    connection: Connection,
    inputPrograms: InputPrograms<Programs>,
    payer?: Keypair,
  }) {
    this.connection = connection;
    this.payer = payer ?? undefined;

    const programs: any = {}
    const tables: any = {}

    for (const entry of Object.values<[PublicKey | string, NautilusIdl]>(inputPrograms)) {
      const [publicKey, program] = entry
      programs[program.name] = publicKey

      for (const table of program.accounts) {
        if (table.config && table.config.tableName) {
          const formattedName = decapitalizeFirstLetter(table.name)
          tables[formattedName] = new NautilusTable(this, new PublicKey(publicKey.toString()), table.name)
        }
      }
    }

    this.programs = programs
    this.tables = tables
  }

  sql(query: string): NautilusQuery {
    return new NautilusQuery(
      this,
      query,
    )
  }
}

export { NautilusQuery, NautilusTable }

import { 
    Connection, 
    Keypair,
    PublicKey, 
} from '@solana/web3.js';
import { 
    NautilusQuery, 
    NautilusQueryBuilder, 
    NautilusTable, 
} from './sql';
import { NautilusUtils } from './util';

export class Nautilus {

    connection: Connection;
    programId: PublicKey;
    payer: Keypair;

    util: NautilusUtils = new NautilusUtils();

    constructor(
        connection: Connection,
        programId: PublicKey,
        payer?: Keypair,
    ) {
        this.connection = connection;
        this.programId = programId;
        if (payer) this.payer = payer;
    }

    table(tableName: string): NautilusTable {
        return new NautilusTable(
            this,
            tableName,
        );
    }

    query(query: string | NautilusQueryBuilder): NautilusQuery {
        return new NautilusQuery(
            this,
            query,
        );
    }
}
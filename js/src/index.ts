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
    PublicKey, 
} from '@solana/web3.js';
import { 
    NautilusQuery, 
    NautilusTable,
} from './sql';
import { NautilusUtils } from './util';

export class Nautilus {

    connection: Connection;
    programs: [PublicKey, string][];
    defaultProgram: PublicKey | undefined;
    payer: Keypair | undefined;

    util: NautilusUtils = new NautilusUtils();

    constructor(
        connection: Connection,
        programs: string | PublicKey | [PublicKey, string][],
        defaultProgram?: string | PublicKey,
        payer?: Keypair,
    ) {
        
        this.connection = connection;
        [this.programs, this.defaultProgram] = parseNautilusArgs(programs, defaultProgram)
        this.payer = payer ?? undefined;
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

function parseNautilusArgs(
    argPrograms: string | PublicKey | [PublicKey, string][],
    argDefaultProgram: string | PublicKey | undefined,
): [[PublicKey, string][], PublicKey | undefined] {

    const checkForDefaultProgram = (found: boolean) => {
        if (!found) throw Error(
            "Instance error: Provided default program was not found in the provided programs list"
        )
    }
    
    let programs: [PublicKey, string][]
    let defaultProgram: PublicKey | undefined = undefined

    if (typeof argPrograms == "string") {
        programs = [[new PublicKey(argPrograms), "default"]]
        if (!argDefaultProgram) defaultProgram = new PublicKey(argPrograms)
    } else if (argPrograms instanceof PublicKey) {
        programs = [[argPrograms, "default"]]
        if (!argDefaultProgram) defaultProgram = argPrograms
    } else {
        programs = argPrograms
        if (argDefaultProgram) {
            if (argDefaultProgram instanceof PublicKey) {
                checkForDefaultProgram(
                    argPrograms.filter(([publicKey, _]) => 
                        publicKey === argDefaultProgram).length != 0
                )
            } else {
                checkForDefaultProgram(
                    argPrograms.filter(([publicKey, name]) => 
                        publicKey.toBase58() == argDefaultProgram || name === argDefaultProgram).length != 0
                )
            }
        } else {
            if (argPrograms.length === 1) defaultProgram = argPrograms[0][0]
        }
    }

    return [programs, defaultProgram]
}
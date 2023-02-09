import { 
    Connection, 
    GetProgramAccountsConfig,
    GetProgramAccountsFilter,
    Keypair, 
    PublicKey, 
    TransactionInstruction,
    VersionedTransaction,
} from '@solana/web3.js';
import { Nautilus } from '../';
import { NautilusUtils } from '../util';

export class NautilusQuery {
    
    nautilus: Nautilus;
    nautilusQuery: NautilusQueryConfig;

    constructor(
        nautilus: Nautilus,
        query: string | NautilusQueryBuilder,
    ) {
        this.nautilus = nautilus;
        this.nautilusQuery = typeof query === 'string'
            ?
            NautilusQuery.parseQueryString(query)
            :
            query.build();
    }

    async execute(): Promise<any[]> {
        return this.nautilusQuery.execute()
    }

    static parseQueryString(queryString: string): NautilusQueryConfig {
        return new NautilusQueryConfig()
    }
}

// TODO: Build a QueryBuilder object that can construct various SQL queries.
//
export class NautilusQueryBuilder {
    
    constructor() {}

    build(): NautilusQueryConfig {
        return new NautilusQueryConfig()
    }
}

// TODO: Build a QueryConfig object that can represent various SQL queries
//      and evaluate itself.
//
export class NautilusQueryConfig {
    constructor() {}
    
    async execute(): Promise<any[]> {
        return NautilusUtils.getProgramAccounts(
            this.nautilusTable,
            this.getProgramAccountsConfig,
        )
    }
}
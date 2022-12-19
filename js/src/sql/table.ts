import { 
    GetProgramAccountsConfig,
    Keypair, 
    TransactionInstruction,
    VersionedTransaction,
} from '@solana/web3.js';
import { Nautilus } from '../';
import { NautilusUtils } from '../util';

export class NautilusTable {

    nautilus: Nautilus;
    tableName: string;

    constructor(
        nautilus: Nautilus,
        tableName: string,
    ) {
        this.nautilus = nautilus;
        this.tableName = tableName;
    }

    create(params: any): NautilusTableWrite {
        return new NautilusTableWrite(
            this,
            NautilusUtils.createCreateInstruction(params),
        )
    }

    delete(params: any): NautilusTableWrite {
        return new NautilusTableWrite(
            this,
            NautilusUtils.createDeleteInstruction(params),
        )
    }

    get(): NautilusTableRead {
        return new NautilusTableRead(this)
    }

    update(params: any): NautilusTableWrite {
        return new NautilusTableWrite(
            this,
            NautilusUtils.createUpdateInstruction(params),
        )
    }
}

class NautilusTableRead {

    nautilusTable: NautilusTable;
    getProgramAccountsConfig: GetProgramAccountsConfig;

    constructor(
        nautilusTable: NautilusTable,
    ) {
        this.nautilusTable = nautilusTable;
        this.getProgramAccountsConfig = {
            // dataSlice:       TODO: Set the data slice based on the table schema
            filters: [],
        };
    }

    async execute(): Promise<any[]> {
        return NautilusUtils.getProgramAccounts(
            this.nautilusTable,
            this.getProgramAccountsConfig,
        )
    }

    where(
        field: string,
        operator: string,
        matches: string,
    ) {
        this.getProgramAccountsConfig.filters?.push(
            NautilusUtils.evaluateWhereFilter(field, operator, matches)
        );
        return this
    }
}

class NautilusTableWrite {

    nautilusTable: NautilusTable;
    transactionInstruction: TransactionInstruction;

    constructor(
        nautilusTable: NautilusTable,
        instruction: TransactionInstruction,
    ) {
        this.nautilusTable = nautilusTable;
        this.transactionInstruction = instruction;
    }

    async execute(signer: Keypair): Promise<string> {
        return NautilusUtils.sendTransactionWithSigner(
            this.nautilusTable.nautilus.connection,
            this.transactionInstruction,
            signer,
        )
    }

    instruction() {
        return this.transactionInstruction
    }

    async transaction(signer: Keypair): Promise<VersionedTransaction> {
        return NautilusUtils.buildTransaction(
            this.nautilusTable.nautilus.connection,
            [this.transactionInstruction],
            signer,
        )
    }
}
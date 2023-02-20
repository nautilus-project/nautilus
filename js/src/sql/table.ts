import { 
    AccountInfo,
    GetProgramAccountsConfig,
    PublicKey, 
    SendOptions, 
    Signer, 
    TransactionInstruction,
} from '@solana/web3.js';
import { Nautilus } from '../';
import { NautilusUtils } from '../util';

enum FetchFirst {
    Delete,
    Update,
}

export class NautilusTable {

    nautilus: Nautilus
    programId: PublicKey | undefined
    tableName: string

    // Reads
    getProgramAccountsConfig: GetProgramAccountsConfig
    returnFields: string[] | undefined
    orderByFunction: any | undefined

    // Writes
    fetchFirst: FetchFirst | undefined
    updateData: any
    instructions: TransactionInstruction[]
    signersList: Signer[]

    constructor(
        nautilus: Nautilus,
        tableName: string,
    ) {
        this.nautilus = nautilus
        if (nautilus.defaultProgram) this.programId = nautilus.defaultProgram
        this.tableName = tableName
        
        this.getProgramAccountsConfig = {
            filters: [],
        }
        this.returnFields = undefined;
        this.orderByFunction = undefined;
        
        this.fetchFirst = undefined;
        this.updateData = undefined;
        this.instructions = []
        this.signersList = []
    }

    // Reads

    fields(returnFields: string[]) { 
        this.returnFields = returnFields 
        return this
    }

    orderBy(field: string, order: string | number) {
        if (order === "ASC" || order === 1) {
            this.orderByFunction = (list: any[]) => list.sort((a, b) => (a[field] > b[field]) ? 1 : -1)
        } else if (order === "DESC" || order === -1) {
            this.orderByFunction = (list: any[]) => list.sort((a, b) => (a[field] > b[field]) ? -1 : 1)
        } else {
            throw Error("Not a valid ordering statement. Can only use \"ASC\" and \"DESC\", or 1 and -1")
        }
        return this
    }

    // TODO: We can optimize this if the only "where" filter is the primary key
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

    async get(): Promise<{
        pubkey: PublicKey,
        account: AccountInfo<any>
    }[]> {
        if (!this.programId) return noProgramIdError()
        return NautilusUtils.getProgramAccounts(
            this.nautilus.connection,
            this.programId,
            this.getProgramAccountsConfig,
            this.returnFields,
        )
    }

    // Writes

    create(data: any | any[]) {
        if (this.programId) {
            const programId = this.programId
            if (Array.isArray(data)) {
                data.forEach((d) => this.instructions.push(
                    NautilusUtils.createCreateInstruction(programId, this.tableName, d)
                ))
            } else {
                this.instructions.push(NautilusUtils.createCreateInstruction(programId, this.tableName, data))
            }
        } else {
            return noProgramIdError()
        }
        return this
    }

    delete() {
        this.fetchFirst = FetchFirst.Delete
        return this
    }

    update(data: any) {
        this.fetchFirst = FetchFirst.Update
        this.updateData = data
        return this
    }

    signers(signers: Signer[]) {
        signers.forEach((s) => this.signersList.push(s))
        return this
    }

    // TODO: Transaction size overflow
    async execute(sendOptions?: SendOptions): Promise<string> {
        if (this.programId) {
            const programId = this.programId
            const instructions = this.instructions
            if (this.fetchFirst) {
                (await this.get()).forEach((account) => this.fetchFirst == FetchFirst.Delete ? 
                    instructions.push(NautilusUtils.createDeleteInstruction(programId, this.tableName, account))
                    :
                    instructions.push(NautilusUtils.createUpdateInstruction(programId, this.tableName, this.updateData))
                )
            }
            return NautilusUtils.sendTransactionWithSigner(
                this.nautilus.connection,
                instructions,
                this.signersList,
                this.signersList[0].publicKey,
                sendOptions,
            )
        } else { 
            return noProgramIdError()
        }
    }
}

const noProgramIdError = () => {
    throw Error("A program ID was not provided in your Nautilus object")
}
import {
    AccountInfo,
    GetProgramAccountsConfig,
    PublicKey,
    SendOptions,
    Signer,
    TransactionInstruction,
} from '@solana/web3.js';
import { AccountType, NautilusTableFieldsName } from '../types';
import { NautilusIdl, NautilusTableConfigIdl, NautilusTableIdl } from '../idl';
import { createCreateInstruction, createDeleteInstruction, createUpdateInstruction, evaluateWhereFilter, getProgramAccounts, sendTransactionWithSigner } from '../util';

import { NautilusProgram } from '../';

enum FetchFirst {
    Delete,
    Update,
}

export class NautilusTable<Program extends NautilusIdl = NautilusIdl, Table extends NautilusTableIdl = NautilusTableIdl> {

    program: NautilusProgram<Program>
    tableName: string
    idl: NautilusTableIdl

    // Reads
    getProgramAccountsConfig: GetProgramAccountsConfig
    returnFields?: NautilusTableFieldsName<Table>
    orderByFunction?: (list: AccountType<Table>[]) => AccountType<Table>[]

    // Writes
    fetchFirst?: FetchFirst
    updateData?: AccountType<Table>
    instructions: TransactionInstruction[]
    signersList: Signer[]

    constructor (
        program: NautilusProgram<Program>,
        idl: NautilusTableIdl
    ) {
        this.program = program
        this.tableName = idl.name
        this.idl = idl

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
    fields(returnFields: NautilusTableFieldsName<Table>) {
        this.returnFields = returnFields
        return this
    }

    orderBy(field: keyof AccountType<Table>, order: "ASC" | "DESC" | 1 | -1) {
        const a = (list: AccountType<Table>[]) => list.sort((a, b) => (a[field] > b[field]) ? 1 : -1)
        a
        if (order === "ASC" || order === 1) {
            this.orderByFunction = (list: AccountType<Table>[]) => list.sort((a, b) => (a[field] > b[field]) ? 1 : -1)
        } else if (order === "DESC" || order === -1) {
            this.orderByFunction = (list: AccountType<Table>[]) => list.sort((a, b) => (a[field] > b[field]) ? -1 : 1)
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
            evaluateWhereFilter(field, operator, matches)
        );
        return this
    }

    async get(): Promise<{
        pubkey: PublicKey,
        account: AccountInfo<AccountType<Table>>
    }[]> {
        if (!this.program.programId) return noProgramIdError()
        return getProgramAccounts(
            this.program.connection,
            this.program.programId,
            this.getProgramAccountsConfig,
            this.returnFields,
        )
    }

    // Writes

    create(data: AccountType<Table> | AccountType<Table>[]) {
        if (this.program) {
            const programId = this.program.programId
            if (Array.isArray(data)) {
                data.forEach((d) => this.instructions.push(
                    createCreateInstruction(programId, this.tableName, d)
                ))
            } else {
                this.instructions.push(createCreateInstruction(programId, this.tableName, data))
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

    update(data: AccountType<Table>) {
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
        if (this.program.programId) {
            const programId = this.program.programId
            const instructions = this.instructions
            if (this.fetchFirst) {
                (await this.get()).forEach((account) => this.fetchFirst == FetchFirst.Delete ?
                    instructions.push(createDeleteInstruction(programId, this.tableName, account))
                    :
                    instructions.push(createUpdateInstruction(programId, this.tableName, this.updateData))
                )
            }
            return sendTransactionWithSigner(
                this.program.connection,
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
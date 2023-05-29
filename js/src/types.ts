import { IdlTypeLookup, NautilusAccountIdl, NautilusAccountIdlConfig, NautilusIdl, NautilusTableIdl } from "./idl"

import { NautilusTable } from "./sql"
import { PublicKey } from "@solana/web3.js"

export type NautilusProgram = {
  tables: { [tableName: string]: string }
}

export type InputPrograms<Programs extends NautilusIdl[] = NautilusIdl[]> = {
  [Program in Programs[number]as Program["name"]]: [PublicKey | string, Program]
}

export type NautilusProgramTables = { [tableName: string]: NautilusTable }

export type NautilusPrograms = NautilusIdl[];

export type AllTableAccounts<Program extends NautilusIdl = NautilusIdl> = Program["accounts"][number] extends NautilusTableIdl ? Required<Program["accounts"][number] | NautilusTableIdl>[] : never

export type AllProgramAccountsConfig<Program extends NautilusIdl = NautilusIdl> = Program["accounts"][number]["config"] extends NautilusAccountIdlConfig ? Program["accounts"][number]["config"][] : never

export type AllProgramTables<Program extends NautilusIdl = NautilusIdl> = AllProgramAccountsConfig<Program>[number]["tableName"] extends NonNullable<string> ? Required<AllProgramAccountsConfig<Program>[number]>[] : never

export type AllProgramTableNames<Program extends NautilusIdl = NautilusIdl> = AllProgramTables<Program>[number]["tableName"] extends string ? AllProgramTables<Program>[number]["tableName"] : never

export type ProgramTables<Program extends NautilusIdl = NautilusIdl> = {
  [Table in AllTableAccounts<Program>[number]as NonNullable<Table["config"]["tableName"]>]: NautilusTable<Program, Table>
}

export type ProgramsTables<Programs extends NautilusIdl[] = NautilusIdl[]> = {
  [Program in Programs[number]as Program["name"]]: ProgramTables<Program>
}

export type NautilusTableFields<Table extends NautilusTableIdl = NautilusTableIdl> = Table["type"]["fields"]

export type NautilusTableFieldsName<Table extends NautilusTableIdl = NautilusTableIdl> = Table["type"]["fields"][number]["name"][]

export type AccountType<Account extends NautilusAccountIdl = NautilusAccountIdl> = {
  [T in Account["type"]["fields"][number]as T["name"]]: IdlTypeLookup[T["type"]]
}

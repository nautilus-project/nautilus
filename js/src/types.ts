import {
  IdlTypeLookup,
  NautilusAccountIdl,
  NautilusAccountIdlConfig,
  NautilusIdl,
  NautilusInstructionIdl,
  NautilusTableIdl,
} from "./idl";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

import { NautilusTable } from "./sql";

export type NautilusProgram = {
  tables: { [tableName: string]: string };
};

export type InputPrograms<Programs extends NautilusIdl[] = NautilusIdl[]> = {
  [Program in Programs[number] as Program["name"]]: [
    PublicKey | string,
    Program
  ];
};

export type NautilusProgramTables = { [tableName: string]: NautilusTable };

export type NautilusPrograms = NautilusIdl[];

export type AllTableAccounts<Program extends NautilusIdl = NautilusIdl> =
  Program["accounts"][number] extends NautilusTableIdl
    ? Required<Program["accounts"][number] | NautilusTableIdl>[]
    : never;

export type AllProgramAccountsConfig<
  Program extends NautilusIdl = NautilusIdl
> = Program["accounts"][number]["config"] extends NautilusAccountIdlConfig
  ? Program["accounts"][number]["config"][]
  : never;

export type AllProgramTables<Program extends NautilusIdl = NautilusIdl> =
  AllProgramAccountsConfig<Program>[number]["tableName"] extends NonNullable<string>
    ? Required<AllProgramAccountsConfig<Program>[number]>[]
    : never;

export type AllProgramTableNames<Program extends NautilusIdl = NautilusIdl> =
  AllProgramTables<Program>[number]["tableName"] extends string
    ? AllProgramTables<Program>[number]["tableName"]
    : never;

export type ProgramTables<Program extends NautilusIdl = NautilusIdl> = {
  [Table in AllTableAccounts<Program>[number] as NonNullable<
    Table["config"]["tableName"]
  >]: NautilusTable<Program, Table>;
};

export type ProgramsTables<Programs extends NautilusIdl[] = NautilusIdl[]> = {
  [Program in Programs[number] as Program["name"]]: ProgramTables<Program>;
};

export type NautilusTableFields<
  Table extends NautilusTableIdl = NautilusTableIdl
> = Table["type"]["fields"];

export type NautilusTableFieldsName<
  Table extends NautilusTableIdl = NautilusTableIdl
> = Table["type"]["fields"][number]["name"][];

export type AccountType<
  Account extends NautilusAccountIdl = NautilusAccountIdl
> = {
  [T in Account["type"]["fields"][number] as T["name"]]: IdlTypeLookup[T["type"]];
};

/** Program instructions **/
export type ProgramInstructions<Program extends NautilusIdl = NautilusIdl> = {
  [Instruction in Program["instructions"][number] as Instruction["name"]]: InstructionBuilder<Instruction>;
};

export type InstructionBuilder<
  Instruction extends NautilusInstructionIdl = NautilusInstructionIdl
> = (
  params: InstructionBuilderParams<Instruction>,
  accounts: InstructionBuilderAccounts<Instruction>
) => TransactionInstruction;

export type InstructionBuilderParams<
  Instruction extends NautilusInstructionIdl = NautilusInstructionIdl
> = {
  [Arg in Instruction["args"][number] as Arg["name"]]: IdlTypeLookup[Arg["type"]];
};

export type InstructionBuilderAccounts<
  Instruction extends NautilusInstructionIdl = NautilusInstructionIdl
> = {
  [Arg in Instruction["accounts"][number] as Arg["name"]]: PublicKey;
};

/** Program seeds **/
export type ProgramSeeds<Program extends NautilusIdl = NautilusIdl> = {
  [Table in AllTableAccounts<Program>[number] as NonNullable<
    Table["config"]
  >["tableName"]]: (primaryKey: PrimaryKeyType<Table>) => PublicKey;
};

// TODO: Fix type
// export type PrimaryKeyType<Table extends NautilusTableIdl = NautilusTableIdl> =
//   Table["type"]["fields"][number]["name"] extends Table["config"]["primaryKey"]
//     ? IdlTypeLookup[Table["type"]["fields"][number]["type"]]
//     : never;

export type PrimaryKeyType<Table extends NautilusTableIdl = NautilusTableIdl> =
  IdlTypeLookup[Table["type"]["fields"][number]["type"]];

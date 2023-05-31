//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//

import * as borsh from "borsh";

import {
  Connection,
  Keypair,
  PublicKey,
  Struct,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  InstructionBuilderAccounts,
  InstructionBuilderParams,
  ProgramInstructions,
  ProgramSeeds,
  ProgramTables,
} from "./types";
import { NautilusIdl, NautilusIdlTypeField, NautilusTableIdl } from "./idl";
import { NautilusQuery, NautilusTable } from "./sql";

import { decapitalizeFirstLetter } from "./util";

export class NautilusProgram<Program extends NautilusIdl = NautilusIdl> {
  connection: Connection;
  programId: PublicKey;
  indexAddress: PublicKey;
  payer?: Keypair;

  readonly seeds: ProgramSeeds<Program>;
  readonly instructions: ProgramInstructions<Program>;
  readonly tables: ProgramTables<Program>;

  constructor({
    connection,
    idl,
    programId,
    payer,
  }: {
    connection: Connection;
    idl: NautilusIdl;
    programId: PublicKey;
    payer?: Keypair;
  }) {
    this.connection = connection;
    this.payer = payer ?? undefined;
    this.programId = programId;
    this.indexAddress = PublicKey.findProgramAddressSync(
      [Buffer.from("nautilus_index"), Buffer.from(Uint8Array.of(0))],
      programId
    )[0];

    const seeds: any = {};
    for (const table of idl.accounts || []) {
      if (table.config && table.config.tableName) {
        const formattedName = decapitalizeFirstLetter(table.name);
        seeds[formattedName] = (primaryKey: any) => {
          return PublicKey.findProgramAddressSync(
            [
              Buffer.from(table.config?.tableName || ""),
              Buffer.from(Uint8Array.of(primaryKey)),
            ],
            this.programId
          )[0];
        };
      }
    }
    this.seeds = seeds;

    const instructions: any = {};
    for (const instruction of idl.instructions || []) {
      instructions[instruction.name] = (
        params: InstructionBuilderParams,
        accounts: InstructionBuilderAccounts
      ) => {
        const keys = instruction.accounts.map((e) => ({
          pubkey: accounts[e.name],
          isSigner: e.isSigner,
          isWritable: e.isMut,
        }));
        const mapPublicKeysToUintArray = () => {
          const res: any = {};
          instruction.args.forEach((arg: NautilusIdlTypeField) => {
            res[arg.name] =
              arg.type === "publicKey"
                ? (params[arg.name] as PublicKey).toBuffer()
                : params[arg.name];
          });
          return res;
        };
        const dataStruct = new Struct({
          instruction: instruction.discriminant.value,
          ...mapPublicKeysToUintArray(),
        });
        const schema = new Map([
          [
            Struct,
            {
              kind: "struct",
              fields: [
                ["instruction", "u8"],
                // TODO: Properly handle types
                ...instruction.args.map((e) => [
                  e.name,
                  e.type === "publicKey" ? [32] : e.type,
                ]),
              ],
            },
          ],
        ]);
        return new TransactionInstruction({
          keys,
          programId: this.programId,
          data: Buffer.from(borsh.serialize(schema, dataStruct)),
        });
      };
    }
    this.instructions = instructions;

    const tables: any = {};
    for (const table of idl.accounts || []) {
      if (table.config && table.config.tableName) {
        const formattedName = decapitalizeFirstLetter(table.name);
        tables[formattedName] = new NautilusTable(
          this,
          table as NautilusTableIdl
        );
      }
    }
    this.tables = tables;
  }

  sql(query: string): NautilusQuery<[Program]> {
    return new NautilusQuery([this], query);
  }
}

export { NautilusQuery, NautilusTable };

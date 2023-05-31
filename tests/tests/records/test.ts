import {
  IDL,
  ProgramNautilusType,
} from "../../programs/records/target/idl/program-nautilus.ts";
import {
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { PAYER, PROGRAM_RECORDS, TEST_CONFIGS } from "../const";
import {
  createCreateCarInstruction,
  createCreateHomeInstruction,
  createCreatePersonInstruction,
  createInitializeInstruction,
  createReadCarInstruction,
  createReadHomeInstruction,
  createReadPersonInstruction,
} from "./instructions";
import { describe, it } from "mocha";

import { NautilusProgram } from "../../../js/src";

describe("Nautilus Unit Tests: Create Records", async () => {
  const connection = TEST_CONFIGS.connection;
  const payer = PAYER;
  const program = PROGRAM_RECORDS;

  const personName = "Joe";
  const homeId = 1;
  const homeHouseNumber = 15;
  const homeStreet = "Solana St.";
  const carMake = "Chevrolet";
  const carModel = "Corvette";
  let nautilus = new NautilusProgram<ProgramNautilusType>({
    connection,
    idl: IDL,
    programId: program.publicKey,
    payer,
  });

  async function test(ix: TransactionInstruction, signers: Keypair[]) {
    TEST_CONFIGS.sleep();
    let sx = await sendAndConfirmTransaction(
      connection,
      new Transaction().add(ix),
      signers,
      { skipPreflight: true }
    );
    console.log(`\n\n  [INFO]: sig: ${sx}\n`);
  }

  it("Initialize Nautilus Index", async () =>
    test(
      nautilus.instructions.initialize(
        {},
        {
          feePayer: payer.publicKey,
          nautilus_index: nautilus.indexAddress,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        }
      ),
      [payer]
    ));

  it("Create Person", async () =>
    test(
      nautilus.instructions.createPerson(
        { name: personName, authority: payer.publicKey },
        {
          new_person: nautilus.seeds.person(1),
          feePayer: payer.publicKey,
          index: nautilus.indexAddress,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        }
      ),
      [payer]
    ));

  it("Read Person", async () =>
    test(
      nautilus.instructions.readPerson(
        { name: personName, authority: payer.publicKey },
        {
          person: nautilus.seeds.person(1),
          index: nautilus.indexAddress,
        }
      ),
      [payer]
    ));

  it("Create Home", async () =>
    test(
      nautilus.instructions.createHome(
        { id: homeId, house_number: homeHouseNumber, street: homeStreet },
        {
          new_home: nautilus.seeds.home(1),
          index: nautilus.indexAddress,
          feePayer: payer.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        }
      ),
      [payer]
    ));

  it("Read Home", async () =>
    test(
      nautilus.instructions.readHome(
        {},
        {
          home: nautilus.seeds.home(1),
          index: nautilus.indexAddress,
        }
      ),
      [payer]
    ));

  it("Create Car", async () =>
    test(
      nautilus.instructions.createCar(
        {
          make: carMake,
          model: carModel,
          purchase_authority: payer.publicKey,
          operating_authority: payer.publicKey,
        },
        {
          new_car: nautilus.seeds.car(1),
          index: nautilus.indexAddress,
          feePayer: payer.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        }
      ),
      [payer]
    ));

  it("Read Car", async () =>
    test(
      nautilus.instructions.readCar(
        {
          make: carMake,
          model: carModel,
          purchase_authority: payer.publicKey,
          operating_authority: payer.publicKey,
        },
        {
          car: nautilus.seeds.car(1),
          index: nautilus.indexAddress,
        }
      ),
      [payer]
    ));
});

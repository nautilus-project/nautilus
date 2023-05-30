import { CONNECTION, PROGRAM_ID } from "./main.test"
import { IDL, ProgramNautilusType } from "../idl/program-nautilus"
import { describe, it } from "mocha"

import { NautilusProgram } from "../src"
import assert from "assert"

export function tests() {

    describe("[Unit Tests]:   Instantiating", () => {
        function canInstantiate(method: string, nautilus: NautilusProgram) {
            it(`   -- Can instantiate:  ${method}`, () => assert(nautilus))
        }

        canInstantiate(
            "pubkey        | no programs",
            new NautilusProgram<ProgramNautilusType>({ connection: CONNECTION, idl: IDL, programId: PROGRAM_ID }),
        )
    })
}
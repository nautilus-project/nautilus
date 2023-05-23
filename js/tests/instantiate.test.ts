import { CONNECTION, PROGRAM_ID, PROGRAM_ID_STRING } from "./main.test"
import { IDL, ProgramNautilusType } from "../idl/program-nautilus"
import { describe, it } from "mocha"

import { Nautilus } from "../src"
import assert from "assert"

export function tests() {

    describe("[Unit Tests]:   Instantiating", () => {

        function canInstantiate(method: string, nautilus: Nautilus) {
            it(`   -- Can instantiate:  ${method}`, () => assert(nautilus))
        }

        canInstantiate(
            "string        | no programs",
            new Nautilus<[ProgramNautilusType]>({ connection: CONNECTION, inputPrograms: { "program-nautilus": [PROGRAM_ID_STRING, IDL] } }),
        )

        canInstantiate(
            "pubkey        | no programs",
            new Nautilus<[ProgramNautilusType]>({ connection: CONNECTION, inputPrograms: { "program-nautilus": [PROGRAM_ID, IDL] } }),
        )
    })
}
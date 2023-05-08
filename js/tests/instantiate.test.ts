import { CONNECTION, PROGRAM_ID, PROGRAM_ID_STRING } from "./main.test"
import { describe, it } from "mocha"

import { Nautilus } from "../src"
import assert from "assert"
import idl from "../idl/program-nautilus.json"

export function tests() {

    describe("[Unit Tests]:   Instantiating", () => {

        function canInstantiate(method: string, nautilus: Nautilus) {
            it(`   -- Can instantiate:  ${method}`, () => assert(nautilus))
        }

        canInstantiate(
            "string        | no programs",
            new Nautilus({ connection: CONNECTION, programId: PROGRAM_ID_STRING, idl }), 
        )

        canInstantiate(
            "pubkey        | no programs",
            new Nautilus({ connection: CONNECTION, programId: PROGRAM_ID, idl }), 
        )

        canInstantiate(
            "pubkey        | single-list",
            new Nautilus({ connection: CONNECTION, programId: PROGRAM_ID, idl, programs: { ["person-program"]: PROGRAM_ID } }),
        )

        canInstantiate(
            "pubkey        | multi-list",
            new Nautilus({ connection: CONNECTION, programId: PROGRAM_ID, idl, programs: { ["person-program"]: PROGRAM_ID, ["person-program-2"]: PROGRAM_ID } }),
        )
    })
}
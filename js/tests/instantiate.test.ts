import assert from "assert"
import { describe, it } from "mocha"
import { Nautilus } from "../src"
import { CONNECTION, PROGRAM_ID, PROGRAM_ID_STRING } from "./main.test"

export function tests() {

    describe("[Unit Tests]:   Instantiating", () => {

        function canInstantiate(method: string, nautilus: Nautilus) {
            it(`   -- Can instantiate:  ${method}`, () => assert(nautilus))
        }

        canInstantiate(
            "string        | no default program",
            new Nautilus(CONNECTION, PROGRAM_ID_STRING), 
        )

        canInstantiate(
            "string        | with default program",
            new Nautilus(CONNECTION, PROGRAM_ID_STRING, PROGRAM_ID), 
        )

        canInstantiate(
            "pubkey        | no default program",
            new Nautilus(CONNECTION, PROGRAM_ID), 
        )

        canInstantiate(
            "pubkey        | with default program",
            new Nautilus(CONNECTION, PROGRAM_ID, PROGRAM_ID), 
        )
        
        canInstantiate(
            "single-list   | no default program",
            new Nautilus(CONNECTION, [[PROGRAM_ID, "person-program"]]),
        )

        canInstantiate(
            "single-list   | with default program  | string arg",
            new Nautilus(
                CONNECTION, 
                [[PROGRAM_ID, "person-program"]],
                "person-program",
            ),
        )

        canInstantiate(
            "single-list   | with default program  | PublicKey arg",
            new Nautilus(
                CONNECTION, 
                [[PROGRAM_ID, "person-program"]],
                PROGRAM_ID,
            ),
        )
        
        canInstantiate(
            "multiple-list | no default program",
            new Nautilus(
                CONNECTION, 
                [
                    [PROGRAM_ID, "person-program"],
                    [PROGRAM_ID, "person-program-2"],
                ],
            ),
        )
        
        canInstantiate(
            "multiple-list | with default program  | string arg",
            new Nautilus(
                CONNECTION, 
                [
                    [PROGRAM_ID, "person-program"],
                    [PROGRAM_ID, "person-program-2"],
                ],
                "person-program",
            ),
        )
        
        canInstantiate(
            "multiple-list | with default program  | PublicKey arg",
            new Nautilus(
                CONNECTION, 
                [
                    [PROGRAM_ID, "person-program"],
                    [PROGRAM_ID, "person-program-2"],
                ],
                PROGRAM_ID,
            ),
        )

    })

}
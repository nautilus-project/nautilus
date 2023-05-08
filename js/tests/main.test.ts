import { Connection, PublicKey } from "@solana/web3.js"

//
// Deploy test program before executing
//
export const CONNECTION = new Connection("http://localhost:8899", "confirmed")
export const PROGRAM_ID_STRING = "Dto5T5KYAsAXeiHZxY1KNDsrrynJrnMMhQJLA5XTVN8R"
export const PROGRAM_ID = new PublicKey("Dto5T5KYAsAXeiHZxY1KNDsrrynJrnMMhQJLA5XTVN8R")

function test(file: string) {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    require(file).tests()
}

//
// Test Suite
//

test("./instantiate.test.ts")
test("./sql-parse.test.ts")
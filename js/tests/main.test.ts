import { Connection, PublicKey } from "@solana/web3.js"

//
// Deploy test program before executing
//
export const CONNECTION = new Connection("http://localhost:8899", "confirmed")
export const PROGRAM_ID_STRING = "9kYnTzxTSTtKJjBBScH2m3SLBq8grogLhwMLZdcD2wG4"
export const PROGRAM_ID = new PublicKey("9kYnTzxTSTtKJjBBScH2m3SLBq8grogLhwMLZdcD2wG4")

function test(file: string) {
    require(file).tests()
}

//
// Test Suite
//

test("./instantiate.test.ts")
test("./sql-parse.test.ts")
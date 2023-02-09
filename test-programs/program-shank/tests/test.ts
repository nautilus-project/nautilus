import {
    it,
    describe,
} from 'mocha'
import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    Transaction,
} from '@solana/web3.js'
import { createCreateHeroInstruction, createDeleteHeroInstruction, createUpdateHeroInstruction } from './instruction'


function loadKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
}


describe("Nautilus Program Unit Tests", () => {

    const connection = new Connection(`https://api.devnet.solana.com`, 'confirmed')
    const payer = loadKeypairFromFile(require('os').homedir() + '/.config/solana/id.json')
    const program = loadKeypairFromFile('./program/target/deploy/program_nautilus-keypair.json')
  
    it("Try CreateHero", async () => {
        let ix = createCreateHeroInstruction(
            payer.publicKey,
            program.publicKey,
            1,
            "Hercules",
            payer.publicKey,
        )
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        )
    })

    it("Try UpdateHero", async () => {
        let ix = createUpdateHeroInstruction(
            payer.publicKey,
            program.publicKey,
            1,
            "Hercules",
            payer.publicKey,
        )
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        )
    })

    it("Try DeleteHero", async () => {
        let ix = createDeleteHeroInstruction(
            payer.publicKey,
            program.publicKey,
        )
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        )
    })
  })
  
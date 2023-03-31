import { Connection, Keypair } from '@solana/web3.js'

export const CONNECTION = new Connection(`https://api.devnet.solana.com`, 'confirmed')
export const PAYER = loadKeypairFromFile(require('os').homedir() + '/.config/solana/id.json')
export const PROGRAM_BASIC = loadKeypairFromFile('./programs/basic/target/deploy/program_nautilus-keypair.json')
export const PROGRAM_CREATE = loadKeypairFromFile('./programs/create/target/deploy/program_nautilus-keypair.json')

function loadKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
}
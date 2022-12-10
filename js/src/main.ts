import { Connection, PublicKey } from '@solana/web3.js';

export async function getAccounts(
    connection: Connection,
    programId: PublicKey,
    publicKey: PublicKey,
) {
    const accounts = await connection.getProgramAccounts(
        programId,
        {
            filters: [
              {
                dataSize: 35,
              },
              {
                memcmp: {
                  offset: 2,
                  bytes: publicKey.toBase58(),
                },
              },
            ],
        },
    );
    return accounts;
}
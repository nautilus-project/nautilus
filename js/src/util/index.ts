import { 
    Connection,
    GetProgramAccountsConfig,
    GetProgramAccountsFilter,
    Keypair,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction,
} from '@solana/web3.js';
import { NautilusTable } from '../sql';

export class NautilusUtils {

    // Get Program Accounts

    static async getProgramAccounts(
        nautilusTable: NautilusTable, 
        config: GetProgramAccountsConfig,
    ): Promise<any[]> {
        return nautilusTable.nautilus.connection.getProgramAccounts(
            nautilusTable.nautilus.programId,
            config,
        )
    }

    // Nautilus Instruction Utils

    static evaluateWhereFilter(
        field: string,
        operator: string,
        matches: string,
    ): GetProgramAccountsFilter {
        return {
            memcmp: {
                offset: 0,
                bytes: 'string',
              }
        }
    }

    // TODO: Build these functions so that they can craft a CUD instruction 
    //      for the user's Solana program, based on the table schema
    //
    static createCreateInstruction(params: any): TransactionInstruction {
        return ix;
    }

    static createUpdateInstruction(params: any): TransactionInstruction {
        return ix;
    }

    static createDeleteInstruction(params: any): TransactionInstruction {
        return ix;
    }


    // Solana Transaction Utils

    static async buildTransaction(
        connection: Connection,
        instructionsList: TransactionInstruction[],
        signer: Keypair,
    ): Promise<VersionedTransaction> {
        const tx = new VersionedTransaction(
            new TransactionMessage({
                payerKey: signer.publicKey,
                recentBlockhash: (
                    await connection
                        .getLatestBlockhash()
                        .then((res) => res.blockhash)
                ),
                instructions: instructionsList,
            }).compileToV0Message()
        );
        tx.sign([signer])
        return tx
    }

    static async sendTransactionWithSigner(
        connection: Connection,
        instruction: TransactionInstruction,
        signer: Keypair,
    ): Promise<string> {
        return connection.sendTransaction(
            (await NautilusUtils.buildTransaction(
                connection, 
                [instruction], 
                signer,
            )),

        )
    }
}
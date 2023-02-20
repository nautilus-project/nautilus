import { 
    AccountInfo,
    Connection,
    GetProgramAccountsConfig,
    GetProgramAccountsFilter,
    PublicKey,
    SendOptions,
    Signer,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction,
} from '@solana/web3.js';

export class NautilusUtils {

    // Get Program Accounts

    static async getProgramAccounts(
        connection: Connection,
        programId: PublicKey, 
        config: GetProgramAccountsConfig,
        returnFields?: string[]
    ): Promise<{
        pubkey: PublicKey,
        account: AccountInfo<any>
    }[]> {
        return connection.getProgramAccounts(
            programId,
            config,
        )
    }

    // Nautilus Instruction Utils

    // TODO: Create these filters based on the IDL and the passed tableName
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

    // TODO: Create these instructions based on the IDL and the passed tableName
    static createCreateInstruction(programId: PublicKey, tableName: string, data: any): TransactionInstruction {
        return {
            programId,
            keys: [{
                pubkey: PublicKey.unique(), isSigner: true, isWritable: true
            }],
            data: Buffer.alloc(0),
        }
    }

    // TODO: Create these instructions based on the IDL and the passed tableName
    static createDeleteInstruction(programId: PublicKey, tableName: string, account: any): TransactionInstruction {
        return {
            programId,
            keys: [{
                pubkey: PublicKey.unique(), isSigner: true, isWritable: true
            }],
            data: Buffer.alloc(0),
        }
    }

    // TODO: Create these instructions based on the IDL and the passed tableName
    static createUpdateInstruction(programId: PublicKey, tableName: string, data: any): TransactionInstruction {
        return {
            programId,
            keys: [{
                pubkey: PublicKey.unique(), isSigner: true, isWritable: true
            }],
            data: Buffer.alloc(0),
        }
    }


    // Solana Transaction Utils

    static async buildTransaction(
        connection: Connection,
        instructionsList: TransactionInstruction[],
        signers: Signer[],
        payerKey: PublicKey,
    ): Promise<VersionedTransaction> {
        const tx = new VersionedTransaction(
            new TransactionMessage({
                payerKey,
                recentBlockhash: (
                    await connection
                        .getLatestBlockhash()
                        .then((res) => res.blockhash)
                ),
                instructions: instructionsList,
            }).compileToV0Message()
        );
        tx.sign(signers)
        return tx
    }

    static async sendTransactionWithSigner(
        connection: Connection,
        instructions: TransactionInstruction[],
        signers: Signer[],
        feePayer: PublicKey,
        sendOptions?: SendOptions,
    ): Promise<string> {
        return connection.sendTransaction(
            (await NautilusUtils.buildTransaction(
                connection, 
                instructions, 
                signers,
                feePayer,
            )),
            sendOptions,
        )
    }
}
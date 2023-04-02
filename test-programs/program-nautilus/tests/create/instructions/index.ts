export * from './associated-token'
export * from './mint'
export * from './metadata'
export * from './token'
export * from './wallet'

export enum MyInstructions {
    CreateWallet,
    CreateWalletWithPayer,
    CreateMint,
    CreateMintWithPayer,
    CreateMetadata,
    CreateMetadataWithPayer,
    CreateAssociatedToken,
    CreateAssociatedTokenWithPayer,
    CreateToken,
    CreateTokenWithPayer,
    TransferWallet,
}
---
title: "Tokens"
description: "Solana SPL Tokens & NFTs"
previous: "Wallets"
previousLink: "/docs/wallets"
next: "Tables"
nextLink: "/docs/tables"
---

---

Tokens in Nautilus programs can be represented in a variety of ways, including two useful aggregator objects:

- `Token` will allow you to create SPL tokens and their metadata all in one command
- `Nft` is similar to `Token` but with some added features and functionality specific to NFTs

Let's start out by considering a `Mint`, which is simply an SPL token mint without any associated metadata.

You can create a `Mint` the same way you'd create any other Nautilus object - like the `Wallet` example in the previous section:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_mint<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))
    }

    fn create_mint_with_payer<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }
}
```

The account state data for `Mint` objects uses the [SPL Token state for `Mint`](https://docs.rs/spl-token/latest/spl_token/state/struct.Mint.html). Here's an example of reading the data of a `Mint`:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn read_mint(mint: Mint) -> ProgramResult {

        info!(" * Mint Public Key: {}", &mint.key());
        print_mint_data(&mint.data);
        Ok(())
    }
}

fn print_mint_data(data: &MintState) {
    info!(" * Mint Data:");
    info!("      Mint Authority:         {:#?}", data.mint_authority);
    info!("      Supply:                 {}", data.supply);
    info!("      Decimals:               {}", data.decimals);
    info!("      Is Initialized:         {}", data.is_initialized);
    info!("      Freeze Authority:       {:#?}", data.freeze_authority);
}
```

Once you've created a `Mint`, or if you have some other existing SPL token mint, you can create only the `Metadata` for a token like so:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_metadata<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_metadata.create(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
        )
    }

    fn create_metadata_with_payer<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_metadata.create_with_payer(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
            rent_payer,
        )
    }
}
```

This will effectively create metadata for any SPL token so long as you are authorized to do so.

The account state data for `Metadata` objects uses the [Metaplex state for `Metadata`](https://docs.rs/mpl-token-metadata/latest/mpl_token_metadata/state/struct.Metadata.html). Here's an example of reading the data of a `Metadata`:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn read_metadata(metadata: Metadata) -> ProgramResult {

        info!(" * Metadata Public Key: {}", &metadata.key());
        print_metadata_data(&metadata.data);
        Ok(())
    }
}

fn print_metadata_data(data: &MetadataState) {
    info!(" * Metadata Data:");
    info!("      Mint:                   {:#?}", data.mint);
    info!(
        "      Primary Sale Happened:  {}",
        data.primary_sale_happened
    );
    info!("      Is Mutable:             {}", data.is_mutable);
    info!("      Edition Nonce:          {:#?}", data.edition_nonce);
    info!("      Title:                  {}", data.data.name);
    info!("      Symbol:                 {}", data.data.symbol);
    info!("      URI:                    {}", data.data.uri);
}
```

Let's now explore the `Token` object, which combines these two types of accounts so you can create tokens with metadata using one object.

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_token<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )
    }

    fn create_token_with_payer<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_token.create_with_payer(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }
}
```

`Token` combines a `Mint` account and a `Metadata` account, so you can access both accounts' state like you would in the examples above:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn read_token(token: Token) -> ProgramResult {

        info!(" * Token Public Key: {}", &token.key());
        print_mint_data(&token.mint.data);
        print_metadata_data(&token.metadata.data);
        Ok(())
    }
}

fn print_mint_data(data: &MintState) {
    info!(" * Mint Data:");
    info!("      Mint Authority:         {:#?}", data.mint_authority);
    info!("      Supply:                 {}", data.supply);
    info!("      Decimals:               {}", data.decimals);
    info!("      Is Initialized:         {}", data.is_initialized);
    info!("      Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState) {
    info!(" * Metadata Data:");
    info!("      Mint:                   {:#?}", data.mint);
    info!(
        "      Primary Sale Happened:  {}",
        data.primary_sale_happened
    );
    info!("      Is Mutable:             {}", data.is_mutable);
    info!("      Edition Nonce:          {:#?}", data.edition_nonce);
    info!("      Title:                  {}", data.data.name);
    info!("      Symbol:                 {}", data.data.symbol);
    info!("      URI:                    {}", data.data.uri);
}
```

Lastly, `Nft` behaves very similar to `Token`, with a few important features:

- Minting is disabled after 1
- Metadata can be expanded to `MasterEdition`, etc.

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_nft<'a>(
        mut new_nft: Create<'a, Nft<'a>>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_nft.create(
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )
    }

    fn create_nft_with_payer<'a>(
        mut new_nft: Create<'a, Nft<'a>>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {

        new_nft.create_with_payer(
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }
}
```

Just like `Token`, you can access the NFT's state data like so:

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn read_nft(nft: Nft) -> ProgramResult {

        info!(" * NFT Public Key: {}", &nft.key());
        print_mint_data(&nft.mint.data);
        print_metadata_data(&nft.metadata.data);
        Ok(())
    }
}

fn print_mint_data(data: &MintState) {
    info!(" * Mint Data:");
    info!("      Mint Authority:         {:#?}", data.mint_authority);
    info!("      Supply:                 {}", data.supply);
    info!("      Decimals:               {}", data.decimals);
    info!("      Is Initialized:         {}", data.is_initialized);
    info!("      Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState) {
    info!(" * Metadata Data:");
    info!("      Mint:                   {:#?}", data.mint);
    info!(
        "      Primary Sale Happened:  {}",
        data.primary_sale_happened
    );
    info!("      Is Mutable:             {}", data.is_mutable);
    info!("      Edition Nonce:          {:#?}", data.edition_nonce);
    info!("      Title:                  {}", data.data.name);
    info!("      Symbol:                 {}", data.data.symbol);
    info!("      URI:                    {}", data.data.uri);
}
```

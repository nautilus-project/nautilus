# Nautilus

<p width="full" margin="auto"><img src="https://raw.githubusercontent.com/nautilus-project/nautilus/main/docs/icons/nautilus-icon.jpg" alt="youtube" width="150" margin="auto" bg="white"/></p>

🚧 **Note**:   
This framework is still under active development and this spec is subject to change.   
Consider this document a general outline of the goals of this project for the time being.

---

The ⛴️ Nautilus Framework is a brand-new Solana program framework for 🦀 Rust. It introduces:
* 🦀 **On-Chain** (Rust):
    * [📥 Object-oriented on-chain programming](#📥-object-oriented-on-chain-programming)
    * [🔗 SQL-native program data](#🔗-sql-native-program-data-pdas)
* 👾 **Client-Side** (TypeScript):
    * [🔍 Native SQL support for program data](#🔍-native-sql-support-for-program-data)
    * [⚡️ Account resolution](#⚡️-account-resolution)

Some other notable elements:
* 🗝️ Dynamic CPI Support
* 🗝️ High-quality error logs
* 🗝️ TypeScript-native client-side types
* 🗝️ More verbose IDL

## 🦀 On-Chain (Rust)

### Overview

Entrypoint:
```rust
// Entrypoint:
#[nautilus]
mod program {
    fn some_instruction(..) -> ProgramResult {}
    fn some_other_instruction(..) -> ProgramResult {}
}

// State:
#[derive(Nautilus)]
struct MyData {
    #[primary_key]
    id: u8,
    value: u8,
}
```

### 📥 Object-Oriented On-Chain Programming

Similar to the [🐴 Seahorse Framework](https://seahorse-lang.org/), Nautilus takes an object-oriented approach to writing on-chain program logic.   
With this approach, one can simply call operations on these objects via methods, such as `transfer`.
```rust
#[nautilus]
mod program {
    fn my_wallet_function(from: Wallet, to: Wallet, amount: u64) -> ProgramResult {

        from.transfer(to, amount)
    }

    fn my_token_function(mint: Token, from: Wallet, to: Wallet, amount: u64) -> ProgramResult {

        mint.transfer(from, to, amount)
    }
}
```

A `Create<T>` wrapper is used for creating new objects, but the concept is still the same as above.
```rust
#[nautilus]
mod program {
    fn create_token(new_token: Create<Token>, decimals: u8) -> ProgramResult {

        new_token.create(decimals)
    }

    fn create_token_with_metadata(
        new_token: Create<Token>, 
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
    ) -> ProgramResult {

        new_token.create(decimals)?;
        new_token.create_metadata(title, symbol, uri)?;
        // - or -
        new_token.create_with_metadata(
            decimals,
            title,
            symbol,
            uri,
        )?;
        Ok(())
    }
}
```

As you can infer, all accounts are abstracted away from you and packed into your instruction automatically based on the operations you conduct on these objects.   
   
But what about custom data (PDAs)? 👇🏼

### 🔗 SQL-Native Program Data (PDAs)

Nautilus provides high-level abstractions around seeds and data relationships, which provide a much more familiar, SQL-based approach to Solana program data.   
   
The supported integrations are as follows:
| Integration | Definition |
|:------------|:-----------|
|[Default](#default)|This table has the default configuration (no custom `authority` applied) and has `autoincrement` **enabled**.|
|[Default Non-Autoincrement](#default-non-autoincrement)|This table has the default configuration (no custom `authority` applied) but `autoincrement` is **disabled**, so you'll need to provide a primary key.|
|[Authority-Protected](#authority-protected)|Regardless of `autoincrement`, each record for this table requires a signature from the `authority` to modify.|
|[Multiple Authority-Protected](#multiple-authority-protected)|Similar to `Authority-Protected`, each record requires a signature from **each** `authority` to modify.|

#### Default
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key]
    id: u32,
    name: String,
}
```
#### Default Non-Autoincrement
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key(autoincrement = false)]
    id: u32,
    name: String,
}
```

Creating `Default` and `Default Non-Autoincrement` PDAs is the same as seen above in your program's code.
```rust
#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, name: String) -> ProgramResult {

        new_person.create(name)
    }
}
```

However, on the client side, you will have to provide the value for `id` (`primary_key`) if autoincrement is disabled.


#### Authority-Protected
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}
```
#### Multiple Authority-Protected
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key(autoincrement = false)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
    #[authority]
    second_authority: Pubkey,
}
```

As you can probably guess, these authority fields will required signatures to modify these records.
   
You might be wondering 🤔, why does this relation matter? 👇🏼

## 👾 Client-Side (TypeScript)

### Overview

```typescript
import { myProgram } from '../target/nautilus/myProgram'

const nautilus = new Nautilus(connection, myProgram)

// Fetching data
let account = nautilus.get(address)                         // Any address, returns byte array non-deserialized data
let person = nautilus.where('Person').address(address)      // Address field, returns deserialized data of type Person

// Sending transactions
nautilus.myProgramInstruction(
    param1,
    param2,
)
.feePayer(payer)
.execute()
// - or -
nautilus.utils.buildTransaction(
    arbitraryInstructions,      // TransactionInstruction[]
    payer,                      // Signer
)
.execute()
```

### 🔍 Native SQL Support for Program Data

Because of Nautilus's concept of relational program data, the client-side SDK can make use of powerful optimizations for loading and querying data.   
   
First, we can filter against `getProgramAccounts` using fields within our data records.
```typescript
let persons: Person[] = nautilus.where('Person').get()

// SQL:
let personsNamedJoe = nautilus.where('Person').eq('name', 'Joe').get()
let personsNamedJoeOrDan = nautilus.where('Person').eq('name', ['Joe', 'Dan']).get()

// SQL & GraphQL:
let personsNamedJoe = nautilus.where('Person').eq('name', 'Joe').get({
    name: string!,
    someOtherField: string?,
})
let personsNamedJoeOrDan = nautilus.where('Person').eq('name', ['Joe', 'Dan']).get({
    name: string!,
    someOtherField: string?,
})
```

### ⚡️ Account Resolution

Nautilus will make every effort to resolve required accounts while allowing you to override it's resolutions where desired.   
   
Take for example the following instruction for transferring an SPL Token, where the associated token accounts can be automatically derived.
```rust
#[nautilus]
mod program {
    fn my_token_function(from: Wallet, to: Wallet, amount: u64) -> ProgramResult {

        from.transfer(to, amount)
    }
}
```

Nautilus can also use your data's **defined relations** (foreign keys) to provide lookup methods.
```typescript
nautilus.myTokenFunction(
    fromPublicKey,          // Public key of the wallet (ATA is not required, but resolved for you)
    toPublicKey,            // Public key of the wallet (ATA is not required, but resolved for you)
    amount,                 // Automatic conversion from `number` to `BN`
)
.feePayer(payer)
.signers([fromKeypair])
.execute()
```

This same concept also applies to PDAs, so you no longer have to worry about deriving the address off-chain **to perform a write**.
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key]
    id: u32,
    name: String,
}

#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>) -> ProgramResult {

        new_person.create(name)
    }
}
```
```typescript
nautilus.createPerson(
    name,
)
.feePayer(payer)
.execute()
```

Creation of accounts defaults to using the transaction fee-payer as the rent-payer, but you can override this.
```rust
#[derive(Nautilus)]
struct Person {
    #[primary_key]
    id: u32,
    name: String,
}

#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, rent_payer: Wallet) -> ProgramResult {

        new_person.create_with_payer(name, rent_payer)
    }
}
```
```typescript
nautilus.createPerson(
    name,
    rentPayer,
)
.feePayer(payer)
.signers([rentPayer])
.execute()
```

## 🗝️ Notes on Other Notable Elements

`<coming soon>`

# 🤔 How Does Nautilus Work?

`<coming soon>`

> A full explanation of how the framework works will be provided as features are enabled.   
> Right now as of writing, many proof-of-concepts have been built in Rust, inside and out of this repository, and the remainder of the spec has been fleshed out on plain 'ol pen and paper.   
> Expect this document to update regularly in the coming weeks. 💪🏼
# ⛴️ Nautilus

🚧 **Note**:   
This framework is still under active development and this spec is subject to change.   
Consider this document a general outline of the goals of this project for the time being.

---

The ⛴️ Nautilus Framework is a brand-new Solana program framework for 🦀 Rust. It introduces:
* 🦀 **On-Chain** (Rust):
    * [📥 Object-oriented on-chain programming](#📥-object-oriented-on-chain-programming)
    * [🔗 Relational program data](#🔗-relational-program-data-pdas)
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
```rust
#[derive(Nautilus)]
struct MyData {
    value: u8
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

    fn my_token_function(from: Wallet, to: Wallet, amount: u64) -> ProgramResult {

        from.transfer(to, amount)
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
        Ok(())
    }
}
```

As you can infer, all accounts are abstracted away from you and packed into your instruction automatically based on the operations you conduct on these objects.   
   
But what about custom data (PDAs)? 👇🏼

### 🔗 Relational Program Data (PDAs)

Nautilus provides high-level abstractions around seeds and data relationships, which provide a much more familiar, SQL-based approach to Solana program data.   
   
The supported integrations are as follows:
| Integration | Definition | Seeds |
|:------------|:-----------|:------|
|[Default Non-Unique](#default-non-unique)|This account has no foreign key relation to other data types within this program, but there can be many records.|`<prefix> + <id>`|
|[Default Unique](#default-unique)|This account has no foreign key relation to other data types within this program, and **there can only be one** record of its kind.|`<prefix>`|
|[Related Non-Unique](#related-non-unique)|This account has **at least one** foreign key relation to other data types within this program, but there can be many records.|`<prefix> + <foreign-key> + <id>`|
|[Related Unique](#related-unique)|This account has **at least one** foreign key relation to other data types within this program, and **there can only be one** record of its kind.|`<prefix> + <foreign-key>`|

#### Default Non-Unique
```rust
#[derive(Nautilus)]
struct Person {
    name: String,
}
```
#### Default Unique
```rust
#[derive(Nautilus)]
#[relation(
    unique
)]
struct Person {
    name: String,
}
```

Creating `Default` PDAs is the same as seen above.
```rust
#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, name: String) -> ProgramResult {

        new_person.create(name)
    }
}
```

#### Related Non-Unique
```rust
#[derive(Nautilus)]
#[relation(
    foreign_keys(
        mint: Pubkey,
    ),
)]
struct Person {
    name: String,
}
```
#### Related Unique
```rust
#[derive(Nautilus)]
#[relation(
    unique,
    foreign_keys(
        mint: Pubkey,
    ),
)]
struct Person {
    name: String,
}
```

When a foreign key is introduced, the `.create(..)` method changes.
```rust
#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, name: String, mint: Token) -> ProgramResult {

        new_person.create(name, (mint.key()))   // Takes 2 parameters now: ( name: String, foreign_keys: (mint: Pubkey) )
    }
}
```
Same thing with more than one foreign key.
```rust
#[derive(Nautilus)]
#[relation(
    unique,
    foreign_keys(
        wallet: Pubkey,
        mint: Pubkey,
    ),
)]
struct Person {
    name: String,
}
```
```rust
#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, name: String, mint: Token, wallet: Wallet) -> ProgramResult {

        new_person.create(name, (wallet.key(), mint.key()))   // Parameters: ( name: String, foreign_keys: (wallet: Pubkey, mint: Pubkey) )
    }
}
```
> Foreign keys are passed into `.create(..)` as a tuple.   
   
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

Nautilus can also use your data's **defined relations** (foreign keys) to provide lookup methods.
```typescript
let personsForWalletAndMint: Person[] = nautilus.where('Person').relatedRecords(wallet, mint)

// Or, for unique:
let personForWalletAndMint: Person = nautilus.where('Person').relatedUniqueRecord(wallet, mint)
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
```typescript
nautilus.myTokenFunction(
    fromPublicKey,          // Public key of the wallet
    toPublicKey,            // Public key of the wallet
    amount,                 // Automatic conversion from `number` to `BN`
)
.feePayer(payer)
.signers([fromKeypair])
.execute()
```

This same concept also applies to PDAs, so you no longer have to worry about deriving the address off-chain **to perform a write**.
```rust
#[derive(Nautilus)]
#[relation(
    unique,
    foreign_keys(
        wallet: Pubkey,
        mint: Pubkey,
    ),
)]
struct Person {
    name: String,
}

#[nautilus]
mod program {
    fn create_person(new_person: Create<Person>, name: String, mint: Token, wallet: Wallet) -> ProgramResult {

        new_person.create(name, (wallet.key(), mint.key()))
    }
}
```
```typescript
nautilus.createPerson(
    name,
    mint,
    wallet,
)
.feePayer(payer)
.execute()
```

## 🗝️ Notes on Other Notable Elements

`<coming soon>`

# 🤔 How Does Nautilus Work?

`<coming soon>`

> A full explanation of how the framework works will be provided as features are enabled.   
> Right now as of writing, many proof-of-concepts have been built in Rust, inside and out of this repository, and the remainder of the spec has been fleshed out on plain 'ol pen and paper.   
> Expect this document to update regularly in the coming weeks. 💪🏼
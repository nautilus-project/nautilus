# :boat: Nautilus

SOL server for Solana

## Contributors:

> Contributors welcome!  
> Feel free to branch/fork this repo and submit improvements!  
> You can find details on how Nautilus works below.

## Nautilus

Nautilus has 3 main components:
* `/cli` : Responsible for handling CLI commands like `cargo build-bpf` and one crucial setup step defined below under [CLI](#CLI).
* `/solana` : The Rust crate that provides the `#[derive(Nautilus)]` and `#[nautilus]` macros.
* `/js` : The client-side SDK for executing CRUD operations on your on-chain data from your dApp.

### :scroll: Nautilus Data Mapping

> To best understand how Nautilus works, let's start with an overview of how it's on-chain data mapping works.

Each PDA account for a Nautilus program follows this specific pattern for it's Program Derived Address (PDA):
```shell
seeds = <table name> + <id value>
```
Or, more specifically, where `TABLE_NAME` is the name of the struct plus string `"_table"`:
```rust
fn address(&self) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[ 
            Self::TABLE_NAME.as_bytes().as_ref(),
            self.id.to_le_bytes().as_ref(),
        ],
        self.program_id
    )
}
```

### :heavy_plus_sign: How AutoIncrement Works

Nautilus leverages what's called a `Counter` account for each table specified by the program. For example, in the case of a struct `Person`:
```rust
pub struct Person {
    id: u32,
    name: String,
}
```
As discussed above, we know we'll be creating new "records" into this table by creating accounts using this schema and the following PDA derivation:
```shell
seeds = "person_table" + <id value (u32)>
```
However, we will **only once** create the following account:
```rust
pub struct Counter {
    count: u32,
}
```
```shell
seeds = "person_table_counter"
```
As you can probably predict, every time we create new "records" in the table, we update this counter. These update functions are not shown below under [the `derive` macro description](#the-derivenautilus-macro).

### :blue_book: Repository Layout
```text
/cli        -- Nautilus CLI
/js         -- Client-side SDK
/solana     -- Solana program crate
    /derive     -- derive(Nautilus) macro
    /src        -- Nautilus crate core (traits & functions)
    /syn        -- Parsing operations for the derive(Nautilus) macro
    /tests      -- Nautilus crate tests
/test       -- Test repositories
```

### :gear: The `#[derive(Nautilus)]` Macro

The macro itself consists of a `proc_derive` macro and it's associated attribute.   
The attribute macro requires a primary key to be specified with `primary_key = <field_name>`:
```rust
#[derive(Nautilus)]
#[nautilus(
    primary_key = id,
)]
pub struct Person {
    id: u32,
    name: String,
}
```
The following **optional** arguments can be passed to the attribute macro:
```rust
#[derive(Nautilus)]
#[nautilus(
    create,
    update,
    delete,
    primary_key = id,
    auto_increment = true,
    authority = authority,
)]
pub struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}
```
Let's break these down:
* `create` : Implement the `NautilusCreate` trait for the struct, which will write the **CreatePersonInstruction** for the program.
* `update` : Implement the `NautilusUpdate` trait for the struct, which will write the **UpdatePersonInstruction** for the program.
* `delete` : Implement the `NautilusDelete` trait for the struct, which will write the **DeletePersonInstruction** for the program.
* `primary_key` : Which field we're going to be using as the object's ID, and therefore also it's **PDA derivation seeds**.
* `auto_increment` : Enables or disables autoincrementing of the primary key by adding the logic to check the [Counter account](#how-autoincrement-works) to auto-increment the ID field. **Primary Key must be a number to use autoincrement**.
* `authority` : Add signer checks to verify that a specific record's `authority` has signed the instruction.

> As you can see, the derive(Nautilus) macro will add all of these functions to each struct, but what about the program's **entrypoint** & **processor**?  
> See [CLI](#nautilus-cli)

### :computer: Nautilus CLI
The CLI is a straightforward wrapper around the following command relationships:
```shell
nautilus build  = cargo build-bpf
nautilus clean  = cargo clean
nautilus deploy = solana program deploy
```
However, the `nautilus build` step is where we introduce extra functionality to perform the creation of the Solana program's **entrypoint** & **processor**.
1. Read all of the developer's Rust code and parse out every struct name and it's specified keywords (`create`, `update`, `delete`).
2. Use this list to write a new Rust file called `nautilus_mod.rs` containing an enum and processor as defined below.
3. Add this line: `mod nautilus_mod` to the developer's `lib.rs`.
4. Run `cargo build-bpf` on the modified codebase.
5. Remove `nautilus_mod.rs` and the extra line in the developer's `lib.rs`.
   
**Enum**
```rust
NautilusProgramInstruction {
    CreatePerson(CreatePersonArgs),
    UpdatePerson(UpdatePersonArgs),
    DeletePerson(DeletePersonArgs),
}
```
**Processor**
```rust
use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = NautilusProgramInstruction::try_from_slice(&instruction_data)?;

    match instruction {

        NautilusProgramInstruction::CreatePerson(args) => {
            msg!("Nautilus Program Instruction: CreatePerson");
            return Person::create(program_id, accounts, args)
        }
        NautilusProgramInstruction::UpdatePerson(args) => {
            msg!("Nautilus Program Instruction: UpdatePerson");
            return Person::update(program_id, accounts, args)
        }
        NautilusProgramInstruction::DeletePerson(args) => {
            msg!("Nautilus Program Instruction: DeletePerson");
            return Person::delete(program_id, accounts, args)
        }
    }
}
```
> This method for adding unseen code to a developer's codebase is less than preferable and is a temporary solution for now.

### :satellite: Client-Side SDK
The client-side SDK is still in development, but generally, the IDL generated by Nautilus will also generate for you TypeScript types that the client-side SDK will rely on to provide functionality.
```typescript
import nautilus from '@nautilus/js'

nautilus.setConnection('https://api.devnet.solana.com', 'confirmed');
```
```typescript
nautilus.table('person')
    .create({
        name: "Joe",
    })
    .signers([payer])
    .execute();
```
```typescript
const ix = nautilus.table('person')
    .create({
        name: "Joe",
    })
    .signers([payer])
    .instruction();

let tx = nautilus.util.createTransaction(
    instructions: [ix],
    feePayer: payer
);
await sendTransaction(tx);
```
```typescript
nautilus.table('person')
    .get()
    .execute();
```
```typescript
const peopleNamedJoe = nautilus.table('person')
    .get()
    .where('name', '==', 'Joe')
    .execute();
```
```typescript
const peopleNamedJoe = nautilus
    .query('SELECT * FROM person WHERE name == "Joe"')
    .execute();
```
# ⛴️ Nautilus

[📝 Features & Roadmap Doc](https://funny-fur-524.notion.site/Nautilus-e9335efcc6cd46acbdcbf123c234fff3)

### 🦀 Writing a Nautilus Program

```rust
#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreatePerson,
    UpdatePerson,
    DeletePerson,
}

#[derive(NautilusAccount)]
pub struct Person {
    #[primary_key(autoincrement = true)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}
```

### 📜 PDA Data Mapping

Each PDA account for a Nautilus program follows this specific pattern for it's Program Derived Address (PDA):
```shell
seeds = <table name> + <id value>
```
Or, more specifically, where `TABLE_NAME` is the name of the struct plus string `"_table"`.

### ➕ How AutoIncrement Works

Nautilus leverages what's called a `Counter` account for each table specified by the program. For example, in the case of a struct `Person`:
```rust
// Seeds: "person_table" + <id value (u32)>
pub struct Person {
    id: u32,
    name: String,
}

// Re-used struct for each "Counter" account
// Seeds (example): "person_table_counter"
pub struct Counter {
    id: u32,
    name: String,
}
```
As you can probably predict, every time we create new "records" in the table, we update this counter.

### 📘 Repository Layout
```text
/cli        -- CLI
/js         -- Client-side SDK
/solana     -- Solana program crate
    /derive     -- derive(Nautilus) macro
    /src        -- Nautilus crate core (traits & functions)
    /syn        -- Parsing operations for the derive(Nautilus) macro
    /tests      -- Nautilus crate tests
/test       -- Test repositories
```

### ⚙️ The `#[derive(NautilusAccount)]` Macro

The macro itself consists of a `proc_derive` macro and it's associated attributes.   
```rust
#[derive(NautilusAccount)]
pub struct Person {
    #[primary_key(autoincrement = true)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}
```
Attributes:
* `primary_key` : Which field we're going to be using as the object's ID, and therefore also it's **PDA derivation seeds**.
    * `auto_increment` : Enables or disables autoincrementing of the primary key by adding the logic to check the [Counter account](#how-autoincrement-works) to auto-increment the ID field. 
        * Primary Key must be a number to use autoincrement.
        * Autoincrement is enabled by default.
* `authority` : Add signer checks to verify that a specific record's `authority` has signed the instruction.
    * Supports multiple `authority` attributes for "multi-sig".

Nautilus will implement the `NautilusCrud` type for your struct, giving it the following features:
* Default `create()`, `update()`, and `delete()` operations
    * These leverage `invoke_signed` and add checks based on the `authority` attributes provided.
* Seed management util functions
    * These leverage `shank` to give developers easy access to a struct's seeds, address, and checks.
* Inner data state management util functions
    * These directly manipulate the account's inner data based on the struct.
    * Think `new()`, `update()`, and `realloc()`.

### :gear: The `#[derive(NautilusEntrypoint)]` Macro

This macro builds your program's entrypoint and processor.   
   
For example, consider the enum provided above:
```rust
#[derive(NautilusEntrypoint)]
NautilusProgramInstruction {
    CreatePerson,
    UpdatePerson,
    DeletePerson,
}
```
The will actually generate the following code:
```rust
use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

NautilusProgramInstruction {
    CreatePerson(CreatePersonArgs),
    UpdatePerson(UpdatePersonArgs),
    DeletePerson(DeletePersonArgs),
}

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
Note the naming conventions of the enum variants dictate the methods called within the processor.   
For example, consider this variant:
```rust
    CreatePerson,
```
The name `CreatePerson` tells Nautilus to use the `create` method from the `Person` struct. This method is automatically implemented for the struct `Person` when the `#[derive(NautilusAccount)]` annotation is added to the Person struct.  
   
If someone wanted to define their own instruction, or override one of the defaults, they just need to provide custom args, like so:
```rust
#[derive(NautilusEntrypoint)]
NautilusProgramInstruction {
    CreatePerson,
    UpdatePerson(MyCustomUpdatePersonArgs),
    DeletePerson,
}
```
Then when they define their actual operation for that instruction variant, it looks like this:
```rust
struct MyCustomUpdatePersonArgs {
    is_paul: bool,
}

fn update_person(program_id: &Pubkey, accounts: &[AccountInfo], args: MyCustomUpdatePersonArgs) {
    
    // The developer's custom logic can still make use of methods and associated functions for the struct
    //      that implements the NautilusCrud trait
    let accounts_iter = accounts.iter();
    let person = Person::from_account_info(accounts_iter.next_account_info());
    
    if (is_paul) {
        person.update(None, Some("Paul"), None); // Updates only the `name` field
    };

    Ok(())
}
```

### 📡 Client-Side SDK
Nautilus makes use of the Shank-generated IDL to create the client-side SDK types and functions, just like Anchor and Solita.

**Initializing**
```typescript
import nautilus from '@nautilus/js'
import { Connection } from '@solana/web3.js'

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const nautilus = new Nautilus(connection);
nautilus.addProgram('MyProgram', '../idl.json');
```
Note Nautilus also supports multiple programs in the same instance:
```typescript
const nautilus = new Nautilus(connection);
nautilus.addProgram('MyProgram', '../idl.json');
nautilus.addProgram('MyProgram2', '../idl2.json');
```
You can set the default program to avoid having to specify on each call:
```typescript
nautilus.addProgram('MyProgram', '../idl.json');
nautilus.setDefault('MyProgram');
// or
nautilus.addDefault('MyProgram', '../idl.json');
```
**Writing Data**
```typescript
nautilus
    .program('MyProgram')
    .table('person')
    .create({
        name: "Joe",
        twitterHandle: "realbuffalojoe",
    })
    .signers([payer])
    .execute();
```
```typescript
const ix = nautilus
    .program('MyProgram')
    .table('person')
    .create({
        name: "Joe",
        twitterHandle: "realbuffalojoe",
    })
    .signers([payer])
    .instruction();

let tx = nautilus.util.createTransaction(
    instructions: [ix],
    feePayer: payer
);
await sendTransaction(tx);
```
Note Nautilus' util functions support Versioned Transactions.

**Reading Data**
```typescript
const allPeople = nautilus
    .program('MyProgram')
    .table('person')
    .get()
    .execute();
```
**Querying Data with SQL**

Use simple selects or joins:
```typescript
const peopleNamedJoe = nautilus
    .program('MyProgram')
    .table('person')
    .where('name', '==', 'Joe')
    .get()
    .execute();
```
```typescript
const peopleNamedJoe = nautilus
    .program('MyProgram')
    .table('person')
    .innerJoin('address', 'address_id')
    .where('name', '==', 'Joe')
    .get()
    .execute();
```
Or make use of raw SQL syntax:
```typescript
const peopleNamedJoe = nautilus
    .program('MyProgram')
    .query('SELECT * FROM person WHERE name == "Joe"')
    .execute();
```
```typescript
const peopleNamedJoe = nautilus
    .program('MyProgram')
    .query('SELECT * FROM person INNER JOIN address (address_id) WHERE name == "Joe"')
    .execute();
```
**Querying Data with GraphQL**
```typescript
const allPeopleJustNames = nautilus.table('person')
    .get()
    .schema({
        name: string!,
    })
    .execute();
```
```typescript
const peopleNamedJoeJustNames = nautilus.table('person')
    .get()
    .schema({
        name: string!,
    })
    .where('name', '==', 'Joe')
    .execute();
```
> Note: Querying data across multiple programs is still a spec, but should also be possible.
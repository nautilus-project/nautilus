---
title: "Tables"
description: "Relational Tables of Solana Data"
previous: "Installation"
previousLink: "/docs/installation"
next: "Javascript SDK"
nextLink: "/docs/javascript-sdk"
---

---

One of Nautilus' core innovations is the use of program-owned accounts with custom data as relational data tables.

Because of Solana's shared state model, it can be tricky to manage your program's own data. Nautilus makes this easier by introducing the concept of **Tables** and **Records**.

Just like a traditional relational database - like PostgreSQL and MySQL - Nautilus allows you to declare, create, and interact with program data as if you were working with a database.

There are a few important things to note when working with Nautilus tables:
* All structs declared as tables must have a **primary key**
* You can choose to enable or disable **autoincrement** of your table's primary key
    * Autoincrement is enabled by default
    * Autoincrement requires a number type for the table's primary key
    * If autoincrement is disabled, you can use other types for the primary key, such as strings and public keys
* You can add one or more **authorities** to any record of the table
    * An authority assigns row-level security to any records in the table
    * A record can only be modified if all assigned authorities have provided signatures
* Tables manage program-derived address seeds automatically

To declare a struct as a Nautilus Table, simply annotate the struct with `#[derive(Table)]`:
```rust
#[derive(Table)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
```

Once you've provided this annotation, you can use the `Person` data type within a `Record<>` encapsulation to tell Nautilus to operate within the `Person` table:
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {
    
    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {

        new_person.create(name, authority)
    }
}
```

As you can see, the fields we've provided in our struct are automatically required as parameters to create a new record. However, notice the **primary key** is not required. This is because we have enabled `autoincrement`, and it will be created for us!

If we were to disable `autoincrement`, you'll see that it's now required.
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {
    
    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        id: u8,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {

        new_person.create(id, name, authority)
    }
}

#[derive(Table)]
struct Person {
    #[primary_key(autoincrement = false)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
```

Notice in both cases we are providing a value for `authority`. As mentioned above, this will store that public key in the record itself, and since the `#[authority]` attribute was provided, Nautilus will always look to validate a signature for that address whenever the record is attempting to be modified.

You can provide more than one `#[authority]` to add even more row-level security to any record.
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {
    
    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        id: u8,
        name: String,
        authority1: Pubkey,
        authority2: Pubkey,
        authority3: Pubkey,
        authority4: Pubkey,
    ) -> ProgramResult {

        new_person.create(
            id, 
            name, 
            authority1,
            authority2,
            authority3,
            authority4,
        )
    }
}

#[derive(Table)]
struct Person {
    #[primary_key(autoincrement = false)]
    id: u8,
    name: String,
    #[authority]
    authority1: Pubkey,
    #[authority]
    authority2: Pubkey,
    #[authority]
    authority3: Pubkey,
    #[authority]
    authority4: Pubkey,
}
```
---
title: "State"
description: "Traditional Non-Record PDAs"
previous: "Tables"
previousLink: "/docs/tables"
next: "Javascript SDK"
nextLink: "/docs/javascript-sdk"
---

---

**State** is a Nautilus concept best suited for develoeprs who are familiar with Solana's Accounts Model and the associated concept of Program-Derived Addresses (PDAs).

Declaring a struct with `#[derive(State)]` simply tells Nautilus to treat this data type as a traditional Solana PDA, so you'll have to provide seeds.

```rust
#[derive(State)]
#[seeds(
    "person",               // Literal seed
    authority,              // Self-referencing seed
)]
struct Person {
    name: String,
    #[authority]
    authority: Pubkey,
}
```

Since you are creating seeds for this type of data, Nautilus cannot offer the relational database goodies like it can with the `#[derive(Table)]` annotation, so you won't be able to declare a `primary key` or use `autoincrement`.

However, you can still take advantage of row-level security! This attribute will tell Nautilus to perform the same checks as if it was being used on the `Table` type.

```rust
#[derive(State)]
#[seeds(
    "person",               // Literal seed
    authority,              // Self-referencing seed
)]
struct Person {
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

Seeds can be provided in three ways:

- Literal constants

```rust
#[derive(State)]
#[seeds(
    "person",               // Literal seed
)]
struct Person {
    name: String,
    #[authority]
    authority: Pubkey,
}
```

- Self-referencing fields of the struct itself

```rust
#[derive(State)]
#[seeds(
    "person",               // Literal seed
    authority,              // Self-referencing seed
)]
struct Person {
    name: String,
    #[authority]
    authority: Pubkey,
}
```

- Provided arguments to the program

```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_person<'a>(
        mut new_person: Create<'a, Account<'a, Person>>,
        name: String,
        authority: Pubkey,
        some_pubkey: Pubkey,
    ) -> ProgramResult {

        new_person.create(name, authority, (some_pubkey,)) // Seed parameter required
    }
}

#[derive(State)]
#[seeds(
    "person",               // Literal seed
    some_pubkey: Pubkey,    // Parameter seed
)]
struct Person {
    name: String,
    #[authority]
    authority: Pubkey,
}
```

Notice when a seed is declared as a provided argument to the program, you must include it in the program's arguments, or derive it from somewhere within the program, to pass it into the tuple value of the `create(..)` function for that `Account`.

Seed args are passed into these types of functions as tuples, and the tuple will be of size and type according to the seeds you declare.

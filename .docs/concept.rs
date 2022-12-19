use nautilus::*;

// The focal point is any state objects marked by the `nautilus` attribute:

// Adding this annotation implements the NautilusCrudObject attributes, including the default
//     Create, Update, Delete operations

// However, none of these operations are available via instructions until added to the entrypoint enum.
//     (It keys on struct name for enum variants)

#[nautilus(
    primary_key = id,       // Required
    autoincrement = true,   //  (Optional)
    authority = authority,  //  (Optional)
)]
struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}

// The below derive macro will lay out the following:
    
            // mod nautilus_program;
            //
            // impl NautilusProgram for MyInstructions
            //      fn entrypoint(program_id, accounts, input)
            //         match MyInstruction::try_from_slice(input)
            //
            //
            // 

//     where it will write NautilusProgram's code for entrypoint() based on the enum

#[derive(Nautilus)]
enum MyInstructions {
    CreatePerson,
    UpdatePerson(UpdatePersonInstruction),
    DeletePerson,
}

entrypoint!(MyInstructions::entrypoint)


// Any default CRUD operations activated by the entrypoint enum can be overriden by providing an
//     argument type to the enum variant -> like UpdatePerson above.

// Referring back to the entrypoint() function of the NautilusProgram above, the default match 
//     statement laid out looks like this:

//             match instructions {
//                 MyInstructions::CreatePerson => Person::create()
//                 MyInstructions::DeletePerson => Person::delete()
//             }

// Adding a type of struct as an arg will lay out the following for UpdatePerson:

//                 MyInstruction::UpdatePerson(ix) => UpdatePersonInstruction::update_person(ix)

struct UpdatePersonAccounts {
    person: AccountInfo,
    payer: AccountInfo,
    mint: AccountInfo,
    token_account: AccountInfo,
    system_program: AccountInfo,
}

struct UpdatePersonArgs {
    id: u32,
    name: String,
}

#[nautilus_instruction]
struct UpdatePersonInstruction {
    accounts: UpdatePersonAccounts,
    args: UpdatePersonArgs,
}

fn update_person(instruction: UpdatePersonInstruction) -> ProgramResult {

    let person = Person::new(
        instruction.args.id,
        instruction.args.name,
    );

    assert!(instruction.accounts.person.lamports() == 0);

    assert!(nautilus_util::spl::validate_token_account(
        instructions.accounts.mint,
        instructions.accounts.token_account,
    ));

    nautilus_util::spl::transfer(
        instruction.token_account,
        10,
    )?;

    person.validate_pda(instruction.accounts.person)?;
    person.update(instructions.accounts.person)?;

    Ok()
}

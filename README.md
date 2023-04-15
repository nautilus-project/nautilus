# Nautilus
[![Rust](https://github.com/nautilus-project/nautilus/actions/workflows/rust.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/rust.yml)
[![TypeScript](https://github.com/nautilus-project/nautilus/actions/workflows/node.js.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/node.js.yml)
[![Python](https://github.com/nautilus-project/nautilus/actions/workflows/python-package.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/python-package.yml)

<p width="full" margin="auto" align="center" style = "background:gray"><img src="https://raw.githubusercontent.com/nautilus-project/nautilus/main/docs/public/nautilus-icon.jpg" alt="youtube" width="200" margin="auto" align="center" bg="white"/></p>

**Sample Program:**
```rust
use nautilus::*;

#[nautilus]
pub mod my_program {

    fn transfer_sol(from: Signer<Wallet>, to: Mut<Wallet>, amount: u64) -> ProgramResult {    
        from.transfer_lamports(to, amount)
    }

    fn create_nft(
        new_nft: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        new_nft.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),     // mint_authority
            mint_authority.clone(),     // update_authority
            Some(mint_authority),       // freeze_authority
        )
    }

    fn create_person(new_person: Create<Person>, name: String, authority: Pubkey) -> ProgramResult {
        new_person.create(name, authority)
    }

    fn update_person(person: Person, name: String, authority: Signer<Pubkey>) -> ProgramResult {
        person.update(name, authority)
    }
}

#[nautilus]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
```

### ⚡️ Rust-Analyzer Friendly!

![](docs/imgs/non_mut_ss.png)
![](docs/imgs/non_mut_analyzer_ss.png)


**Example from `test-programs/programs/source-robust`:**
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn create_token(
        new_token: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
        update_authority: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- Token: {}", &new_token.key());
        msg!("-- Metadata: {}", &new_token.metadata().key());
        msg!("-- Update Authority: {}", &update_authority.key());
        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            update_authority,
            Some(mint_authority),
        )
    }

    fn get_token_info(token: Token) -> ProgramResult {
        msg!("-- Token: {}", &token.key());
        msg!("-- Current Token State:");
        msg!("      Decimals:       {}", token.data().decimals);
        msg!("      Title:          {}", token.data().title);
        msg!("      Symbol:         {}", token.data().symbol);
        msg!("      URI:            {}", token.data().uri);
        msg!("      Mint Auth:      {:#?}", token.data().mint_authority);
        msg!("      Freeze Auth:    {:#?}", token.data().freeze_authority);
        msg!("      Update Auth:    {}", token.data().update_authority);

        Ok(())
    }

    fn create_person(new_person: Create<Person>, name: String, authority: Pubkey) -> ProgramResult {
        msg!("-- New Person: {}", &new_person.key());
        new_person.create(name, authority)
    }

    fn read_person(person: Person) -> ProgramResult {
        msg!("-- Person: {}", &person.key());
        msg!("      ID:             {}", person.data().id);
        msg!("      Name:           {}", person.data().name);
        msg!("      Authority:      {}", person.data().authority);
        Ok(())
    }

    fn create_home(
        new_home: Create<Home>,
        id: u8,
        house_number: u8,
        street: String,
    ) -> ProgramResult {
        msg!("-- New Home: {}", &new_home.key());
        new_home.create(id, house_number, street)
    }

    fn read_home(home: Home) -> ProgramResult {
        msg!("-- Home: {}", &home.key());
        msg!("      ID:             {}", home.data().id);
        msg!("      House Number:   {}", home.data().house_number);
        msg!("      Street:         {}", home.data().street);
        Ok(())
    }
}

#[nautilus]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[nautilus]
struct Home {
    #[primary_key(autoincrement = false)]
    id: u8,
    house_number: u8,
    street: String,
}

#[nautilus]
#[default_instructions(Create, Delete, Update)]
struct Car {
    #[primary_key(autoincrement = true)]
    id: u8,
    make: String,
    model: String,
    #[authority]
    purchase_authority: Pubkey,
    #[authority]
    operating_authority: Pubkey,
}

```

### 🔎 How It Works

![](docs/imgs/miro_howitworks.jpg)
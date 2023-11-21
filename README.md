# Nautilus
[![Rust](https://github.com/nautilus-project/nautilus/actions/workflows/rust.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/rust.yml)
[![TypeScript](https://github.com/nautilus-project/nautilus/actions/workflows/typescript.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/typescript.yml)
[![Python](https://github.com/nautilus-project/nautilus/actions/workflows/python.yml/badge.svg)](https://github.com/nautilus-project/nautilus/actions/workflows/python.yml)

<p width="full" margin="auto" align="center" style = "background:gray"><img src="https://raw.githubusercontent.com/nautilus-project/nautilus/main/docs/public/nautilus-icon.jpg" alt="youtube" width="200" margin="auto" align="center" bg="white"/></p>

**Snippets from Tests:**
```rust
use nautilus::*;

#[nautilus]
pub mod my_program {

    fn create<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        new_wallet.create()
    }

    fn transfer<'a>(from: Signer<Wallet<'a>>, to: Mut<Wallet<'a>>, amount: u64) -> ProgramResult {
        from.transfer_lamports(to.clone(), amount)
    }

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

    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        new_person.create(name, authority)
    }

    fn create_home<'a>(
        mut new_home: Create<'a, Account<'a, Home>>,
        house_number: u8,
        street: String,
        some_pubkey: Pubkey,
    ) -> ProgramResult {
        new_home.create(house_number, street, (some_pubkey,)) // Seed parameter required
    }
}

#[derive(Table)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(State)]
#[seeds(
    "home",                 // Literal seed
    some_pubkey: Pubkey,    // Parameter seed
)]
struct Home {
    house_number: u8,
    street: String,
}
```

### ⚡️ Rust-Analyzer Friendly!

![](docs/imgs/non_mut_ss.png)
![](docs/imgs/non_mut_analyzer_ss.png)


**More Snippets from Tests:**
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_wallet.create_with_payer(rent_payer)
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

    fn mint_mint_to<'a>(
        mint: Mut<Mint<'a>>,
        to: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        mint.mint_to(to, authority, amount)
    }

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

    fn create_home<'a>(
        mut new_home: Create<'a, Record<'a, Home>>,
        id: u8,
        house_number: u8,
        street: String,
    ) -> ProgramResult {
        new_home.create(id, house_number, street)
    }

    fn transfer_from_home<'a>(
        home: Mut<Record<'a, Home>>,
        recipient: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        home.transfer_lamports(recipient, amount)
    }

    fn create_car<'a>(
        mut new_car: Create<'a, Account<'a, Car>>,
        make: String,
        model: String,
        purchase_authority: Pubkey,
        operating_authority: Pubkey,
    ) -> ProgramResult {
        new_car.create(make, model, purchase_authority, operating_authority)
    }

    /// A simulated "complex" program instruction to test Nautilus.
    /// The logic herein is just for example.
    fn complex<'a>(
        _authority1: Signer<Wallet<'a>>, // Marking this as `Signer` will ensure it's a signer on the tx.
        authority2: Signer<Wallet<'a>>,
        rent_payer1: Signer<Wallet<'a>>,
        rent_payer2: Signer<Wallet<'a>>,
        wallet_to_allocate: Create<'a, Wallet<'a>>, // Marking this as `Create` will ensure it hasn't been created.
        mut wallet_to_create: Create<'a, Wallet<'a>>,
        wallet_to_create_with_transfer_safe: Create<'a, Wallet<'a>>,
        wallet_to_create_with_transfer_unsafe: Mut<Wallet<'a>>,
        some_other_transfer_recipient: Mut<Wallet<'a>>,
        amount_to_fund: u64,
        amount_to_transfer: u64,
    ) -> ProgramResult {
        //
        // /* Business Logic */
        //

        // Some random checks to simulate how custom checks might look.
        assert!(rent_payer1
            .owner()
            .eq(&nautilus::solana_program::system_program::ID));
        assert!(rent_payer2
            .owner()
            .eq(&nautilus::solana_program::system_program::ID));

        // Even though the check will be applied via `Signer` in the function sig, you can still
        // check yourself if you choose to.
        assert!(authority2.is_signer());

        // Even though the check will be applied via `Create` in the function sig, you can still
        // check yourself if you choose to.
        assert!(wallet_to_allocate.lamports() == 0);
        assert!(wallet_to_allocate.is_writable());
        wallet_to_allocate.allocate()?;

        assert!(wallet_to_create.lamports() == 0);
        assert!(wallet_to_create.is_writable());
        wallet_to_create.create()?;

        // Safe - checked at entry with `Create`.
        rent_payer1.transfer_lamports(wallet_to_create_with_transfer_safe, amount_to_fund)?;

        // Unsafe - not marked with `Create`.
        rent_payer2.transfer_lamports(wallet_to_create_with_transfer_unsafe, amount_to_fund)?;

        // Transfer with balance assertions
        let from_beg_balance = authority2.lamports();
        let to_beg_balance = some_other_transfer_recipient.lamports();
        authority2.transfer_lamports(some_other_transfer_recipient.clone(), amount_to_transfer)?;
        let from_end_balance = authority2.lamports();
        let to_end_balance = some_other_transfer_recipient.lamports();
        assert!(from_beg_balance - from_end_balance == amount_to_transfer);
        assert!(to_end_balance - to_beg_balance == amount_to_transfer);
        //
        Ok(())
    }
}

#[derive(Table)]
struct Home {
    #[primary_key(autoincrement = false)]
    id: u8,
    house_number: u8,
    street: String,
}


#[derive(State)]
#[seeds(
    "car",                  // Literal seed
    purchase_authority,     // Self-referencing seed
    operating_authority,    // Self-referencing seed
)]
struct Car {
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

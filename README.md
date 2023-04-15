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
https://github.com/nautilus-project/nautilus/blob/87a086cccb9763df30ba3c3786323ed6dd088e0a/test-programs/programs/source-robust/src/lib.rs#L1-L102

### 🔎 How It Works

![](docs/imgs/miro_howitworks.jpg)
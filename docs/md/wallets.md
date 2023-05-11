---
title: "Wallets"
description: "Solana System Accounts"
next: "Wallets"
nextLink: "/docs/wallets"
---

---

The **Wallet** object in a Nautilus program simply represents a Solana system account - an account owned by the System Program and created with an ED25519 keypair.

You can create system accounts in a variety of ways, and as long as you've registered the keypair's public address with the Solana network you've deployed your Nautilus program to, it will map to a valid Wallet object!

The most common way to create system accounts on Solana is through the CLI:
```shell
solana-keygen new

solana airdrop 1 --keypair ./path-to-keypair.json
# - or -
solana transfer 1 <NEW_WALLET_ADDRESS>
```

You can also create a system account using `@solana/web3.js`, or using a Solana program!

To create a system account in a Nautilus program, you can do the following:
* This method will create a system account with the transaction fee payer as the rent payer for this new account:
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {

        new_wallet.create()
    }
}
```
* This method will create a system account and allow you to specify the rent payer for this new account:
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
}
```

You can use system accounts in your program for whatever you want, including requiring their signature.
```rust
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn transfer<'a>(from: Signer<Wallet<'a>>, to: Mut<Wallet<'a>>, amount: u64) -> ProgramResult {
        
        from.transfer_lamports(to, amount)
    }
}
```
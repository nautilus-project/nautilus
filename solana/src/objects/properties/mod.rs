use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use self::mutable::NautilusMut;

pub mod create;
pub mod mutable;
pub mod record;
pub mod signer;

/// The core trait that marks an object in a Nautilus program as being comprised of Solana accounts.
///
/// Basically, if a struct implements this trait, that means its built using one or more `Box` pointers to
/// accounts passed into the program, ie. `Box<AccountInfo<'_>>`
///
/// The implementation of this trait allows the instance of the implemented struct to be used in many methods and functions across
/// the Nautilus source library.
pub trait NautilusAccountInfo<'a>: Clone {
    /// Returns a `Box` pointer to the `AccountInfo` representing the underlying account.
    ///
    /// Some Nautilus objects (structs) may have multiple `Box<AccountInfo<'_>>` fields, but this method should make
    /// its best effort to return the account most closely associated with the object in question.
    ///
    /// For example, a `nautilus::Token` contains two non-program `Box<AccountInfo<'_>>` fields - one for the Mint and one for the Metadata.
    /// The account returned by this method for a `nautilus::Token` is the Mint.
    fn account_info(&self) -> Box<AccountInfo<'a>>;

    /// Returns a reference to the public key representing the underlying account.
    ///
    /// Similar to the `account_info(&self)` method, this method should make its best effort to return
    /// the public key of the account most closely associated with the object in question.
    ///
    /// For example, a `nautilus::Token` contains two non-program `Box<AccountInfo<'_>>` fields - one for the Mint and one for the Metadata.
    /// The public key returned by this method for a `nautilus::Token` is the address of the Mint.
    fn key(&self) -> &'a Pubkey;

    /// Similar to the `account_info(&self)` and `key(&self)` methods, this returns whether or not the closest associated underlying
    /// `AccountInfo` is a signer.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn is_signer(&self) -> bool;

    /// Similar to the `account_info(&self)` and `key(&self)` methods, this returns whether or not the closest associated underlying
    /// `AccountInfo` is mutable.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn is_writable(&self) -> bool;

    /// Returns the lamports balance of the closest associated underlying `AccountInfo`.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn lamports(&self) -> u64;

    /// Returns a mutable reference to the lamports balance of the closest associated underlying `AccountInfo`.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError>;

    /// Returns the address of the owner of the closest associated underlying `AccountInfo`.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn owner(&self) -> &'a Pubkey;

    /// Returns the span (data length) of the closest associated underlying `AccountInfo`.
    ///
    /// For more context on what "closest associated underlying" means, see the documentation for `account_info(&self)` or `key(&self)`.
    fn span(&self) -> Result<usize, ProgramError>;

    /// Converts the `span` to a u64.
    fn size(&self) -> Result<u64, ProgramError> {
        Ok(self.span()?.try_into().unwrap())
    }

    /// The amount of Lamports required to pay rent for the particular data type associated with this Nautilus object.
    fn required_rent(&self) -> Result<u64, solana_program::program_error::ProgramError> {
        use solana_program::sysvar::Sysvar;
        Ok((solana_program::sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()?))
    }
}

/// This is a standalone trait since its really only available to system accounts or PDAs owned by your program.
///
/// One should be careful when conducting `assign` operations, as changing the program owner of an account can
/// have unwanted consequences and/or can be irreversible.
pub trait NautilusAssignable<'a>: NautilusAccountInfo<'a> {
    /// Assign ownership of an account to some program.
    fn assign(&self, owner: &'a Pubkey) -> ProgramResult;
}

/// Since different types of Solana accounts vary in how they conduct transfers depending on their designated owner,
/// this trait allows for varying implementations depending on the type of account associated with a Nautilus object.
///
/// For example, a system account would have to CPI to the System Program to conduct a transfer, but a PDA would not.
pub trait NautilusTransferLamports<'a>: NautilusAccountInfo<'a> {
    /// Conducts a transfer of Lamports from this object (its underlying account) to the designated recipient.
    fn transfer_lamports(self, to: impl NautilusMut<'a>, amount: u64) -> ProgramResult;
}

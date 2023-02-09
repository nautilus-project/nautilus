use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn crud_impl(struct_name: &Ident) -> TokenStream {
    quote! {
        impl #struct_name {

            fn test_crud() {
                println!("Test");
            }

        }
    }
}

// --------------------------------------------------------------------
// Sample traits to be implemented:
//
// pub trait NautilusAccountInner {
//     fn new_inner() -> Self;
//     fn update_inner() -> Self;
// }
// pub struct NautilusAccountCreateArgs {
//     id: u32,
//     name: String,
//     authority: Pubkey,
// }
// pub trait NautilusAccountCreate {
//     fn create(
//         program_id: &Pubkey,
//         accounts: &[AccountInfo],
//         args: NautilusAccountCreateArgs,
//     ) -> ProgramResult;
// }
// pub trait NautilusAccountDelete {
//     fn delete(
//         program_id: &Pubkey,
//         accounts: &[AccountInfo],
//     ) -> ProgramResult;
// }
// pub struct NautilusAccountUpdateArgs {
//     id: Option<u32>,
//     name: Option<String>,
//     authority: Option<Pubkey>,
// }
// pub trait NautilusAccountUpdate {
//     fn update(
//         program_id: &Pubkey,
//         accounts: &[AccountInfo],
//         args: NautilusAccountUpdateArgs,
//     ) -> ProgramResult;
// }

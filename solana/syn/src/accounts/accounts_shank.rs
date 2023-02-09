use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn shank_impl(struct_name: &Ident) -> TokenStream {
    quote! {
        impl #struct_name {

            fn test_shank() {
                println!("Test");
            }

        }
    }
}

// --------------------------------------------------------------------
// Sample traits to be implemented:
//
// pub trait NautilusAccountShank {
//     fn shank_seeds<'a>() -> [&'a [u8]; 8];  // Nusize
//     fn shank_seeds_with_bump<'a>() -> [&'a [u8]; 8];    // Nusize
//     fn shank_pda<'a>() -> (Pubkey, u8);
//     fn shank_pda_with_bump<'a>() -> (Pubkey, u8);
// }
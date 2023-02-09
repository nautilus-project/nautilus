use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn borsh_impl(struct_name: &Ident) -> TokenStream {
    quote! {
        use borsh::{ BorshDeserialize, BorshSerialize };

        impl BorshDeserialize for #struct_name {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                BorshDeserialize::deserialize(&mut &buf[..])
            }
        }

        impl BorshSerialize for #struct_name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                BorshSerialize::serialize(self, writer)
            }
        }

        impl NautilusAccountBorsh for #struct_name {}
    }
}
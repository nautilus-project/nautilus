pub struct SpawnNautilusEntrypoint {
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
}

impl SpawnNautilusEntrypoint {
    pub fn from_ast(ast: &crate::NautilusEntrypointEnum) -> Self {
        let name = ast.ident.clone();
        let variants = match &ast.data {
            syn::Data::Enum(syn::DataEnum { variants, .. }) => variants.clone(),
            _ => panic!("Expected an enum"),
        };

        Self { name, variants }
    }

    pub fn generate(&self) -> proc_macro2::TokenStream {
        let self_nautilus_entrypoint_borsh_tokens =
            super::borsh::nautilus_entrypoint_borsh(&self.name, &self.variants);
        let self_nautilus_processor_tokens =
            super::processor::nautilus_processor(&self.name, &self.variants);
        quote::quote! {
            // #self_nautilus_entrypoint_borsh_tokens

            entrypoint!(process_instruction);

            fn process_instruction(
                program_id: &Pubkey,
                accounts: &[AccountInfo],
                input: &[u8],
            ) -> ProgramResult {
                #self_nautilus_processor_tokens
            }
        }
        .into()
    }
}

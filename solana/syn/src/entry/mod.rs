//
//
// ----------------------------------------------------------------
//                 Nautilus entrypoint token generation
// ----------------------------------------------------------------
//
//     proc_macro2::TokenStream -> ItemMod
//         -> NautilusEntrypoint -> * -> proc_macro2::TokenStream
//                                  * New tokens created here
//
//
mod entry_enum;
mod entry_variant;
mod required_account;

#[derive(Debug)]
pub struct NautilusEntrypoint {
    pub mod_ident: syn::Ident,
    pub mod_content: Vec<syn::Item>,
    pub nautilus_enum: entry_enum::NautilusEntrypointEnum,
    pub crate_context: shank_macro_impl::krate::CrateContext,
}

impl From<syn::ItemMod> for NautilusEntrypoint {
    fn from(value: syn::ItemMod) -> Self {
        let root = std::env::current_dir().unwrap().join("src/lib.rs");
        let crate_context = shank_macro_impl::krate::CrateContext::parse(root).expect(
            "Failed to detect `src/lib.rs`. Are you sure you've built your program with `--lib` ?",
        );
        let nautilus_structs = parse_nautilus_structs(crate_context.structs().cloned().into_iter());
        let mod_ident = value.ident;
        let mod_content = value.content.unwrap().1;
        let nautilus_enum = entry_enum::NautilusEntrypointEnum::new(
            nautilus_structs,
            mod_content.iter().filter_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    Some(item_fn.clone())
                } else {
                    None
                }
            }),
        );
        Self {
            mod_ident,
            mod_content,
            nautilus_enum,
            crate_context,
        }
    }
}

impl syn::parse::Parse for NautilusEntrypoint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(syn::ItemMod::parse(input)?.into())
    }
}

impl quote::ToTokens for NautilusEntrypoint {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&NautilusEntrypoint> for proc_macro2::TokenStream {
    fn from(ast: &NautilusEntrypoint) -> Self {
        let mod_ident = &ast.mod_ident;
        let content = &ast.mod_content;

        let nautilus_enum = &ast.nautilus_enum;
        // nautilus_enum.write_idl_fragment();

        let (instruction_enum, processor) = nautilus_enum.into();

        quote::quote! {
            use #mod_ident::*;
            #instruction_enum
            #processor
            pub mod #mod_ident {
                use nautilus::*;
                #(#content)*
            }
        }
        .into()
    }
}

fn parse_nautilus_structs(
    structs: impl Iterator<Item = syn::ItemStruct>,
) -> Vec<(String, Vec<required_account::RequiredAccount>)> {
    let mut parsed_structs: Vec<(String, Vec<required_account::RequiredAccount>)> = structs
        .filter_map(|s| {
            if let Some(attr) = s.attrs.iter().find(|attr| attr.path.is_ident("derive")) {
                if let Ok(meta) = attr.parse_meta() {
                    if let syn::Meta::List(meta_list) = meta {
                        if meta_list.nested.iter().any(|nested| {
                            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                                path.is_ident("Nautilus")
                            } else {
                                false
                            }
                        }) {
                            return Some(s.ident.to_string());
                        }
                    }
                }
            }
            None
        })
        .map(|s| {
            (
                s,
                required_account::RequiredAccount::get_default_create_required_accounts(),
            )
        })
        .collect();
    parsed_structs.extend(required_account::RequiredAccount::get_default_nautilus_structs());
    parsed_structs
}

//
//
// ----------------------------------------------------------------
//                 Nautilus entrypoint token generation
// ----------------------------------------------------------------
//
//
mod entry_enum;
mod entry_variant;
mod required_account;

#[derive(Debug)]
pub struct NautilusEntrypoint {
    pub mod_ident: syn::Ident,
    pub mod_content: Vec<syn::Item>,
    pub instruction_enum: proc_macro2::TokenStream,
    pub processor: proc_macro2::TokenStream,
}

impl From<syn::ItemMod> for NautilusEntrypoint {
    fn from(value: syn::ItemMod) -> Self {
        let mod_ident = value.ident;
        let mod_content = value.content.unwrap().1;

        let (crate_version, crate_name) = parse_manifest();
        let (nautilus_object_names, idl_accounts, idl_types) = parse_crate_context();

        let nautilus_enum = &entry_enum::NautilusEntrypointEnum::new(
            mod_ident.clone(),
            nautilus_object_names,
            mod_content.iter().filter_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    Some(item_fn.clone())
                } else {
                    None
                }
            }),
        );
        let (instruction_enum, processor, idl_instructions) = nautilus_enum.into();

        nautilus_idl::Idl::new(
            &crate_version,
            &crate_name,
            idl_instructions,
            idl_accounts,
            idl_types,
            nautilus_idl::IdlMetadata::new("TODO!"), // TODO: Get program ID
        )
        .write();

        Self {
            mod_ident,
            mod_content,
            instruction_enum,
            processor,
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
        let mod_content = &ast.mod_content;
        let instruction_enum = &ast.instruction_enum;
        let processor = &ast.processor;

        quote::quote! {
            use #mod_ident::*;
            #instruction_enum
            #processor
            pub mod #mod_ident {
                use nautilus::*;
                #(#mod_content)*
            }
        }
        .into()
    }
}

fn parse_manifest() -> (String, String) {
    let manifest = cargo_toml::Manifest::from_path("Cargo.toml")
        .expect("Failed to detect `Cargo.toml`. Is your Cargo.toml file structured properly ?");
    let package = manifest
        .package
        .expect("Failed to parse `Cargo.toml`. Is your Cargo.toml file structured properly ?");
    let crate_version = package
        .version
        .get()
        .expect("Failed to parse crate version from `Cargo.toml`. Did you provide one ?");
    (String::from(crate_version), package.name)
}

fn parse_crate_context() -> (
    Vec<String>,
    Vec<nautilus_idl::IdlAccount>,
    Vec<nautilus_idl::IdlType>,
) {
    let root = std::env::current_dir().unwrap().join("src/lib.rs");
    let crate_context = shank_macro_impl::krate::CrateContext::parse(root).expect(
        "Failed to detect `src/lib.rs`. Are you sure you've built your program with `--lib` ?",
    );

    let mut idl_accounts: Vec<nautilus_idl::IdlAccount> = vec![];
    let mut idl_types: Vec<nautilus_idl::IdlType> = vec![];

    // TODO: Enums & more

    let mut nautilus_structs: Vec<String> = crate_context
        .structs()
        .filter_map(|s| {
            if let Some(attr) = s.attrs.iter().find(|attr| attr.path.is_ident("derive")) {
                if let Ok(meta) = attr.parse_meta() {
                    if let syn::Meta::List(meta_list) = meta {
                        if meta_list.nested.iter().any(|nested| {
                            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
                                path.is_ident("Nautilus")
                            } else {
                                idl_types.push(nautilus_idl::IdlType::new(
                                    &s.ident.to_string(),
                                    idl_type_from_struct_fields(&s.fields),
                                ));
                                false
                            }
                        }) {
                            let account_ident_string = s.ident.to_string();
                            idl_accounts.push(nautilus_idl::IdlAccount::new(
                                &account_ident_string,
                                idl_type_from_struct_fields(&s.fields),
                            ));
                            return Some(account_ident_string);
                        }
                    }
                }
            }
            None
        })
        .collect();
    nautilus_structs.extend(required_account::RequiredAccount::get_source_nautilus_structs());

    (nautilus_structs, idl_accounts, idl_types)
}

fn idl_type_from_struct_fields<'a>(fields: &'a syn::Fields) -> nautilus_idl::IdlTypeType {
    nautilus_idl::IdlTypeType::new(
        "struct",
        match fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_type = format!("{}", quote::quote! { #field.ty });
                    nautilus_idl::IdlTypeTypeField::new(&field_name, &field_type)
                })
                .collect(),
            _ => vec![],
        },
    )
}

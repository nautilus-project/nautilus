use case::CaseExt;

#[derive(Debug)]
pub struct NautilusEntrypointEnumVariant {
    pub mod_ident: syn::Ident,
    pub name: syn::Ident,
    pub required_accounts: Vec<super::required_account::RequiredAccount>,
    pub nautilus_objects: Vec<(
        syn::Ident,
        syn::Type,
        Vec<super::required_account::RequiredAccount>,
    )>,
    pub args: Vec<(syn::Ident, syn::Type)>,
    pub discriminant: u8,
}

impl NautilusEntrypointEnumVariant {
    pub fn new(
        mod_ident: syn::Ident,
        discriminant: u8,
        function: syn::ItemFn,
        nautilus_object_names: Vec<String>,
    ) -> Self {
        let name = syn::Ident::new(
            &convert_case::Casing::to_case(
                &function.sig.ident.to_string(),
                convert_case::Case::Pascal,
            ),
            proc_macro2::Span::call_site(),
        );
        let (required_accounts, nautilus_objects, args) =
            parse_args(function.sig.inputs, nautilus_object_names);
        Self {
            mod_ident,
            name,
            required_accounts,
            nautilus_objects,
            args,
            discriminant,
        }
    }

    pub fn to_idl_instruction(&self) -> nautilus_idl::IdlInstruction {
        nautilus_idl::IdlInstruction {
            name: self.name.to_string().to_camel_lowercase(),
            accounts: self
                .required_accounts
                .iter()
                .map(|a| a.to_idl_instruction_account())
                .collect(),
            args: self
                .args
                .iter()
                .map(|(ident, ty)| {
                    nautilus_idl::IdlInstructionArg::new(
                        &ident.to_string(),
                        nautilus_idl::IdlInstructionArgType::new(&type_to_string(&ty).unwrap()),
                    )
                })
                .collect(),
            discriminant: nautilus_idl::IdlInstructionDiscriminant::new(self.discriminant),
        }
    }
}

impl From<&NautilusEntrypointEnumVariant>
    for (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        nautilus_idl::IdlInstruction,
    )
{
    fn from(value: &NautilusEntrypointEnumVariant) -> Self {
        let name = &value.name;
        let enum_name = super::entry_enum::NautilusEntrypointEnum::enum_name();
        let (arg_names, arg_types): (Vec<syn::Ident>, Vec<syn::Type>) =
            value.args.clone().into_iter().unzip();
        let match_arm_logic = build_match_arm_logic();
        (
            quote::quote! { #name(#(#arg_types,)*), },
            quote::quote! { #enum_name::#name(#(#arg_names,)*) => #match_arm_logic, },
            value.to_idl_instruction(),
        )
    }
}

fn parse_args(
    fn_args: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    nautilus_object_names: Vec<String>,
) -> (
    Vec<super::required_account::RequiredAccount>,
    Vec<(
        syn::Ident,
        syn::Type,
        Vec<super::required_account::RequiredAccount>,
    )>,
    Vec<(syn::Ident, syn::Type)>,
) {
    let mut nautilus_objects = vec![];
    let args = fn_args
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg.clone() {
                if let syn::Pat::Ident(ident) = *pat_type.pat {
                    let ty = *pat_type.ty;
                    if let syn::Type::Path(type_path) = &ty {
                        if let Some(ty_ident) = type_path.path.get_ident() {
                            for s in nautilus_object_names.iter() {
                                //
                                // TODO: Create<T>
                                //
                                if s.contains(&ty_ident.to_string()) {
                                    let ty_as_string = type_to_string(&ty).unwrap();
                                    nautilus_objects.push((
                                        ident.ident.clone(),
                                        ty,
                                        super::required_account::RequiredAccount::resolve_for_read(
                                            ident.ident.to_string(),
                                            super::required_account::RequiredAccount::derive_object_type(
                                                &ty_as_string
                                            ),
                                        ),
                                    ));
                                    return None;
                                }
                            }
                        }
                    }
                    Some((ident.ident.clone(), ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let required_accounts = super::required_account::RequiredAccount::condense(
        nautilus_objects
            .clone()
            .into_iter()
            .map(|(_, _, r)| r)
            .collect(),
    );
    (required_accounts, nautilus_objects, args)
}

fn type_to_string(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

fn build_match_arm_logic() -> proc_macro2::TokenStream {
    quote::quote! { Ok(()) }
}

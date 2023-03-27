#[derive(Debug)]
pub struct NautilusEntrypointEnum {
    pub variants: Vec<super::entry_variant::NautilusEntrypointEnumVariant>,
}

impl NautilusEntrypointEnum {
    pub fn new(
        nautilus_objects: Vec<crate::object::NautilusObject>,
        entrypoint_functions: impl Iterator<Item = syn::ItemFn>,
    ) -> Self {
        let variants = entrypoint_functions
            .enumerate()
            .map(|(i, f)| {
                let (variant_ident, variant_args, call_ident, call_context) =
                    evaluate_function(&nautilus_objects, f);
                super::entry_variant::NautilusEntrypointEnumVariant::new(
                    i.try_into().unwrap(),
                    variant_ident,
                    variant_args,
                    call_ident,
                    call_context,
                )
            })
            .collect();
        Self { variants }
    }

    pub fn enum_ident() -> syn::Ident {
        syn::Ident::new("NautilusEntrypoint", proc_macro2::Span::call_site())
    }
}

impl From<&NautilusEntrypointEnum>
    for (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        Vec<nautilus_idl::IdlInstruction>,
    )
{
    fn from(value: &NautilusEntrypointEnum) -> Self {
        let enum_name = NautilusEntrypointEnum::enum_ident();
        let (variants, match_arms, idl_instructions) = value.variants.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut variants, mut match_arms, mut idl_instructions), v| {
                let (a, b, c): (
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                    nautilus_idl::IdlInstruction,
                ) = v.into();
                variants.push(a);
                match_arms.push(b);
                idl_instructions.push(c);
                (variants, match_arms, idl_instructions)
            },
        );
        (
            quote::quote! {
                #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
                pub enum #enum_name {
                    #(#variants)*
                }
            },
            quote::quote! {
                pub fn process_instruction(
                    program_id: &Pubkey,
                    accounts: &[AccountInfo],
                    input: &[u8],
                ) -> ProgramResult {
                    let instruction = #enum_name::try_from_slice(input)?;

                    match instruction {
                        #(#match_arms)*
                    }
                }

                entrypoint!(process_instruction);
            },
            idl_instructions,
        )
    }
}

fn evaluate_function(
    nautilus_objects: &Vec<crate::object::NautilusObject>,
    function: syn::ItemFn,
) -> (
    syn::Ident,
    Vec<(syn::Ident, syn::Type)>,
    syn::Ident,
    Vec<super::entry_variant::CallContext>,
) {
    let variant_ident = syn::Ident::new(
        &convert_case::Casing::to_case(&function.sig.ident.to_string(), convert_case::Case::Pascal),
        proc_macro2::Span::call_site(),
    );
    let call_ident = function.sig.ident.clone();
    let mut variant_args = vec![];
    let call_context = function
        .sig
        .inputs
        .into_iter()
        .map(|input| match input {
            syn::FnArg::Typed(pat_type) => match *pat_type.pat {
                syn::Pat::Ident(ref pat_ident) => {
                    for obj in nautilus_objects {
                        if obj
                            .ident
                            .to_string()
                            .eq(&crate::util::type_to_string(&pat_type.ty).unwrap())
                        {
                            let mut nautilus_obj = obj.clone();
                            nautilus_obj.arg_ident = Some(pat_ident.ident.clone());
                            return super::entry_variant::CallContext::Nautilus(nautilus_obj);
                        }
                    }
                    variant_args.push((pat_ident.ident.clone(), *pat_type.ty.clone()));
                    return super::entry_variant::CallContext::Arg(pat_ident.ident.clone());
                }
                _ => panic!("Error parsing function."),
            },
            _ => panic!("Error parsing function."),
        })
        .collect();
    (variant_ident, variant_args, call_ident, call_context)
}

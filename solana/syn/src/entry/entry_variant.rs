#[derive(Debug)]
pub struct NautilusEntrypointEnumVariant {
    pub discriminant: u8,
    pub variant_ident: syn::Ident,
    pub variant_args: Vec<(syn::Ident, syn::Type)>,
    pub required_accounts: Vec<super::required_account::RequiredAccount>,
    pub call_ident: syn::Ident,
    pub call_context: Vec<CallContext>,
}

#[derive(Debug)]
pub enum CallContext {
    Nautilus(crate::NautilusObject),
    Arg(syn::Ident),
}

impl NautilusEntrypointEnumVariant {
    pub fn new(
        discriminant: u8,
        variant_ident: syn::Ident,
        variant_args: Vec<(syn::Ident, syn::Type)>,
        call_ident: syn::Ident,
        call_context: Vec<CallContext>,
    ) -> Self {
        let required_accounts = crate::required_account::RequiredAccount::condense(
            call_context
                .iter()
                .filter_map(|ctx| match ctx {
                    CallContext::Nautilus(n) => Some(n.get_required_accounts()),
                    CallContext::Arg(_) => None,
                })
                .collect(),
        );
        Self {
            discriminant,
            variant_ident,
            variant_args,
            required_accounts,
            call_ident,
            call_context,
        }
    }

    fn build_match_arm_logic(&self) -> proc_macro2::TokenStream {
        use crate::required_account::{Construct, RequiredAccountType};
        let all_accounts = self.required_accounts.iter().map(|r| {
            let ident = match &r.account_type {
                RequiredAccountType::Account => match &r.construct {
                    Construct::Metadata(..) => crate::required_account::metadata_ident(&r.ident),
                    Construct::MintAuthority(..) => {
                        crate::required_account::mint_authority_ident(&r.ident)
                    }
                    _ => crate::required_account::self_account_ident(&r.ident),
                },
                _ => r.ident.clone(),
            };
            quote::quote! { let #ident = next_account_info(accounts_iter)?; }
        });
        let mut object_inits = vec![];
        let mut call_args = vec![];
        {
            self.call_context.iter().for_each(|ctx| {
                match ctx {
                    CallContext::Nautilus(obj) => match &obj.arg_ident {
                        Some(arg) => {
                            let obj_type = &obj.ident;
                            let required_accounts_for_obj = obj.get_required_accounts();
                            let initializers = required_accounts_for_obj.iter().map(|r| {
                                let t: proc_macro2::TokenStream = r.into();
                                t
                            });
                            object_inits
                                .push(quote::quote! { let #arg = #obj_type{#(#initializers,)*}; });
                            call_args.push(quote::quote! { #arg })
                        }
                        None => {
                            panic!("Error processing entrypoint: `arg_ident` not set.")
                        }
                    },
                    CallContext::Arg(arg) => call_args.push(quote::quote! { #arg }),
                };
            });
        }
        let call_ident = &self.call_ident;
        quote::quote! {
            {
                let accounts_iter = &mut accounts.iter();
                #(#all_accounts)*
                #(#object_inits)*
                #call_ident(#(#call_args,)*)
            }
        }
    }

    pub fn to_idl_instruction(&self) -> nautilus_idl::IdlInstruction {
        let mut name = self.variant_ident.to_string();
        name.replace_range(..1, &name[..1].to_lowercase());
        nautilus_idl::IdlInstruction {
            name,
            accounts: self
                .required_accounts
                .iter()
                .map(|a| a.to_idl_instruction_account())
                .collect(),
            args: self
                .variant_args
                .iter()
                .map(|(ident, ty)| {
                    nautilus_idl::IdlInstructionArg::new(
                        &ident.to_string(),
                        nautilus_idl::IdlInstructionArgType::new(
                            &crate::util::type_to_string(&ty).unwrap(),
                        ),
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
        let variant_ident = &value.variant_ident;
        let enum_ident = super::entry_enum::NautilusEntrypointEnum::enum_ident();
        let (arg_names, arg_types): (Vec<syn::Ident>, Vec<syn::Type>) =
            value.variant_args.clone().into_iter().unzip();
        let match_arm_logic = value.build_match_arm_logic();
        (
            quote::quote! { #variant_ident(#(#arg_types,)*), },
            quote::quote! { #enum_ident::#variant_ident(#(#arg_names,)*) => #match_arm_logic, },
            value.to_idl_instruction(),
        )
    }
}

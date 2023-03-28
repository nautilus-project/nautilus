use nautilus_idl::{
    IdlInstruction, IdlInstructionArg, IdlInstructionArgType, IdlInstructionDiscriminant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::entry::required_account::{
    metadata_ident, mint_authority_ident, self_account_ident, Construct, RequiredAccountType,
};

use super::{
    call_context::CallContext, entry_enum::NautilusEntrypointEnum, parser::type_to_string,
    required_account::RequiredAccount,
};

#[derive(Debug)]
pub struct NautilusEntrypointEnumVariant {
    pub discriminant: u8,
    pub variant_ident: Ident,
    pub variant_args: Vec<(Ident, Type)>,
    pub required_accounts: Vec<RequiredAccount>,
    pub call_ident: Ident,
    pub call_context: Vec<CallContext>,
}

impl NautilusEntrypointEnumVariant {
    pub fn new(
        discriminant: u8,
        variant_ident: Ident,
        variant_args: Vec<(Ident, Type)>,
        call_ident: Ident,
        call_context: Vec<CallContext>,
    ) -> Self {
        let required_accounts = RequiredAccount::condense(
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

    fn build_match_arm_logic(&self) -> TokenStream {
        let all_accounts = self.required_accounts.iter().map(|r| {
            let ident = match &r.account_type {
                RequiredAccountType::Account => match &r.construct {
                    Construct::Metadata(..) => metadata_ident(&r.ident),
                    Construct::MintAuthority(..) => mint_authority_ident(&r.ident),
                    _ => self_account_ident(&r.ident),
                },
                _ => r.ident.clone(),
            };
            quote! { let #ident = next_account_info(accounts_iter)?; }
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
                                let t: TokenStream = r.into();
                                t
                            });
                            object_inits.push(quote! { let #arg = #obj_type{#(#initializers,)*}; });
                            call_args.push(quote! { #arg })
                        }
                        None => {
                            panic!("Error processing entrypoint: `arg_ident` not set.")
                        }
                    },
                    CallContext::Arg(arg) => call_args.push(quote! { #arg }),
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

    pub fn to_idl_instruction(&self) -> IdlInstruction {
        let mut name = self.variant_ident.to_string();
        name.replace_range(..1, &name[..1].to_lowercase());
        IdlInstruction {
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
                    IdlInstructionArg::new(
                        &ident.to_string(),
                        IdlInstructionArgType::new(&type_to_string(&ty).unwrap()),
                    )
                })
                .collect(),
            discriminant: IdlInstructionDiscriminant::new(self.discriminant),
        }
    }
}

impl From<&NautilusEntrypointEnumVariant> for (TokenStream, TokenStream, IdlInstruction) {
    fn from(value: &NautilusEntrypointEnumVariant) -> Self {
        let variant_ident = &value.variant_ident;
        let enum_ident = NautilusEntrypointEnum::enum_ident();
        let (arg_names, arg_types): (Vec<Ident>, Vec<Type>) =
            value.variant_args.clone().into_iter().unzip();
        let match_arm_logic = value.build_match_arm_logic();
        (
            quote! { #variant_ident(#(#arg_types,)*), },
            quote! { #enum_ident::#variant_ident(#(#arg_names,)*) => #match_arm_logic, },
            value.to_idl_instruction(),
        )
    }
}

//! A `syn`-powered struct that dissolves to the required components to create the
//! variants of the program's instruction enum and it's associated processor match arm
//! initialization logic.
use nautilus_idl::idl_instruction::{IdlInstruction};
use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{Ident, Type};

use crate::{
    entry::required_account::{RequiredAccountSubtype, to_ident_pointer}, 
    object::{NautilusObject, source::source_nautilus_names, parser::NautilusObjectConfig}
};

use super::{
    entry_enum::NautilusEntrypointEnum,
    required_account::{
        metadata_ident, mint_authority_ident, self_account_ident, RequiredAccount,
        RequiredAccountType,
    },
};

/// The struct used to house all of the required components for building out the generated program, derived
/// from the user's declared function.
///
/// The key functionality actually occurs in the trait implementations for this struct - including the self implementations such as `new(..)`.
#[derive(Debug)]
pub struct NautilusEntrypointEnumVariant {
    /// Instruction discriminant: derived from the order the functions are declared.
    pub discriminant: u8,
    /// The identifier of this instruction's variant in the program instruction enum.
    pub variant_ident: Ident,
    /// The arguments required for this instruction's variant in the program instruction enum.
    pub variant_args: Vec<(Ident, Type)>,
    /// All required accounts for this instruction, in order to instantiate the declared Nautilus objects.
    pub required_accounts: Vec<RequiredAccount>,
    /// The identifier of the user's declared function, in order to call it.
    pub call_ident: Ident,
    /// The "call context" of each declared parameter in the user's defined function signature.
    /// 
    /// "Call context" can be explored further by examining the documentation for `CallContext`, but 
    /// essentially it's information about whether or not the parameter is a Nautilus object or an instruction argument.
    pub call_context: Vec<CallContext>,
}

/// "Call context" for each declared parameter in the user's defined function signature.
#[derive(Debug)]
pub enum CallContext {
    /// The parameter is in fact a Nautilus object.
    /// 
    /// Houses the configurations for this specific Nautilus object declared, which will tell
    /// Nautilus how to instanitate it.
    Nautilus(NautilusObject),
    /// The parameter is an instruction argument and not a Nautilus object.
    Arg(Ident),
}

impl NautilusEntrypointEnumVariant {
    /// Creates a new `NautilusEntrypointEnumVariant`.
    ///
    /// This action will map each `CallContext::Nautilus(..)` for the parameters declared in the user's function to 
    /// determine all required accounts for the instruction.
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
                    CallContext::Nautilus(n) => {
                        let req = n.get_required_accounts();
                        let mut accounts = vec![];
                        accounts.extend(req.0);
                        match req.1 {
                            Some(r) => accounts.extend(r),
                            None => (),
                        };
                        Some(accounts)
                    }
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

    /// Builds the processor match arm for this particular declared function.
    /// 
    /// This function is where the bulk of the magic occurs.
    /// 
    /// Its basically going to generate the code to extract all required accounts from the provided
    /// list of accounts in the program's entrypoint, ie. `accounts: &[AccountInfo]`, then use those
    /// accounts to create `Box` pointers and instantiate each declared Nautilus object, then call the user's function.
    fn build_match_arm_logic(&self) -> TokenStream {
        let instruction_name = self.variant_ident.to_string();
        let mut index_init = quote!();
        // Maps all required accounts for this instruction into the proper tokens to extract from the iterator and 
        // create a `Box` pointer for that account.
        // The `Box` pointer is created in this step, so all cloning later in the match arm is cloning the `Box<AccountInfo>`
        // instead of the `AccountInfo` itself.
        let all_accounts = self.required_accounts.iter().map(|r| {
            let ident = match &r.account_type {
                RequiredAccountType::Account(subtype) => match &subtype {
                    RequiredAccountSubtype::SelfAccount => self_account_ident(&r.ident),
                    RequiredAccountSubtype::Metadata => metadata_ident(&r.ident),
                    RequiredAccountSubtype::MintAuthority => mint_authority_ident(&r.ident),
                },
                RequiredAccountType::IndexAccount => {
                    index_init = quote! { let nautilus_index = NautilusIndex::load(program_id, index_pointer)?; }; // TODO
                    r.ident.clone()
                }
                _ => r.ident.clone(),
            };
            let ident_pointer = to_ident_pointer(&ident);
            quote! { 
                let #ident = next_account_info(accounts_iter)?.to_owned(); 
                let #ident_pointer = Box::new(#ident); 
            }
        });
        let mut object_inits = vec![];
        let mut call_args = vec![];
        // This block is going to try to instantiate every Nautilus object needed to call the user's function.
        // Non-Nautilus objects will simply pass-through to the called function.
        // The last line of the processor match arm will call the user's function with all of the instantiated "call_args".
        {
            self.call_context.iter().for_each(|ctx| {
                match ctx {
                    CallContext::Nautilus(obj) => match &obj.entry_config {
                        Some(config) => {
                            let arg_ident = &config.arg_ident;
                            let (obj_type, arg_ty, is_custom) = match source_nautilus_names().contains(&obj.ident.to_string()) {
                                true => (obj.ident.clone(), quote!(), false),
                                false => {
                                    let ty = &obj.ident;
                                    (
                                        match &obj.object_config {
                                            Some(t) => match t {
                                                NautilusObjectConfig::RecordConfig { .. } => Ident::new("Record", Span::call_site()),
                                                NautilusObjectConfig::AccountConfig { .. } => Ident::new("Account", Span::call_site()),
                                            },
                                            None => panic!("Object {} did not match any source Nautilus objects and was not annotated with a Nautilus #[derive(..)] macro", &obj.ident.to_string()),
                                        }, 
                                        quote! { #ty },
                                        true,
                                    )
                                },
                            };
                            let required_accounts_for_obj = obj.get_required_accounts();
                            // Identifiers for all accounts required "for read" - in other words, any `Box<AccountInfo<'_>>` fields required
                            // for that Nautilus object.
                            let read_call_idents = required_accounts_for_obj.0.iter().map(|r| {
                                let t: TokenStream = r.into();
                                t
                            });
                            match required_accounts_for_obj.1 {
                                // If the object is wrapped in `Create<'_, T>`, this option will have a value.
                                // This means we need to get the identifiers for all accounts required "for create" as well.
                                Some(accounts_for_create) => {
                                    let create_call_idents = accounts_for_create.iter().map(|r| {
                                        let t: TokenStream = r.into();
                                        t
                                    });
                                    let create_obj_init = match is_custom {
                                        true => quote! { 
                                            let mut #arg_ident = Create::new(
                                                #(#create_call_idents,)*
                                                #obj_type::< #arg_ty >::new(#(#read_call_idents,)*)
                                            )?;
                                        },
                                        false => quote! { 
                                            let mut #arg_ident = Create::new(
                                                #(#create_call_idents,)*
                                                #obj_type::new(#(#read_call_idents,)*)
                                            )?;
                                        },
                                    };
                                    object_inits.push(create_obj_init);
                                },
                                None => {
                                    if config.is_signer { 
                                        object_inits.push(
                                            quote! { let #arg_ident = Signer::new(#obj_type::load(#(#read_call_idents,)*)?)?; },
                                        );
                                    } else if config.is_mut {
                                        object_inits.push(
                                            quote! { let #arg_ident = Mut::new(#obj_type::load(#(#read_call_idents,)*)?)?; },
                                        );
                                    } else { 
                                        object_inits.push(match is_custom {
                                                true => quote! { let #arg_ident = #obj_type::< #arg_ty >::load(#(#read_call_idents,)*)?; },
                                                false => quote! { let #arg_ident = #obj_type::load(#(#read_call_idents,)*)?; },
                                            }
                                        );
                                    }
                                },
                            };
                            
                            call_args.push(quote! { #arg_ident })
                        }
                        None => {
                            panic!("Error processing entrypoint: `entry_config` not set.")
                        }
                    },
                    CallContext::Arg(arg) => call_args.push(quote! { #arg }),
                };
            });
        }
        let call_ident = &self.call_ident;
        quote::quote! {
            {
                splogger::info!("Instruction: {}", #instruction_name);
                let accounts_iter = &mut accounts.iter();
                #(#all_accounts)*
                #index_init
                #(#object_inits)*
                #call_ident(#(#call_args,)*)
            }
        }
    }
}

impl From<&NautilusEntrypointEnumVariant> for (TokenStream, TokenStream, IdlInstruction) {
    /// Dissolves the `NautilusEntrypointEnumVariant` into the proper components for building out the generated program.
    /// 
    /// When the `NautilusEntrypointEnum` is dissolved, it dissolves each `NautilusEntrypointEnumVariant` of its `variants` vector and 
    /// aggregates each generated component.
    ///
    /// Consider the return type of the function itself - defined at the trait level: (`TokenStream`, `TokenStream`, `IdlInstruction`):
    /// * `TokenStream` (first): The identifier and associated arguments for the program instruction enum variant for this particular declared function.
    /// * `TokenStream` (second): The processor match arm for this particular declared function.
    /// * `IdlInstruction`: The IDL instruction derived from this particular declared function.
    fn from(value: &NautilusEntrypointEnumVariant) -> Self {
        let variant_ident = &value.variant_ident;
        let enum_ident = NautilusEntrypointEnum::enum_ident();
        let (arg_names, arg_types): (Vec<Ident>, Vec<Type>) =
            value.variant_args.clone().into_iter().unzip();
        let match_arm_logic = value.build_match_arm_logic();
        (
            quote! { #variant_ident(#(#arg_types,)*), },
            quote! { #enum_ident::#variant_ident(#(#arg_names,)*) => #match_arm_logic, },
            value.into(),
        )
    }
}
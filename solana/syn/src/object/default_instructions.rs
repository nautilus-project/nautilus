use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, ItemFn, NestedMeta};

/// Possible default instructions for records.
#[derive(Clone, Debug)]
pub enum DefaultInstruction {
    Create {
        struct_ident: Ident,
        instruction: String,
    },
    Delete {
        struct_ident: Ident,
        instruction: String,
    },
    Update {
        struct_ident: Ident,
        instruction: String,
    },
}

impl DefaultInstruction {
    pub fn parse(nested_meta: &NestedMeta, struct_ident: &Ident) -> syn::Result<Self> {
        if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = nested_meta {
            let variant_string = path.get_ident().unwrap().to_string();
            if variant_string.eq("Create") {
                return Ok(DefaultInstruction::Create {
                    struct_ident: struct_ident.clone(),
                    instruction: String::from("nautilusDefaultCreate") + &struct_ident.to_string(),
                });
            } else if variant_string.eq("Delete") {
                return Ok(DefaultInstruction::Delete {
                    struct_ident: struct_ident.clone(),
                    instruction: String::from("nautilusDefaultDelete") + &struct_ident.to_string(),
                });
            } else if variant_string.eq("Update") {
                return Ok(DefaultInstruction::Update {
                    struct_ident: struct_ident.clone(),
                    instruction: String::from("nautilusDefaultUpdate") + &struct_ident.to_string(),
                });
            } else {
                return Err(syn_error(&format!(
                    "Unknown default instruction: {}",
                    variant_string
                )));
            }
        } else {
            return Err(syn_error(
                "Invalid format for `default_instructions` attribute",
            ));
        }
    }
}

pub struct DefaultInstructionParser {
    pub instructions: Vec<DefaultInstruction>,
}

impl DefaultInstructionParser {
    pub fn parse(attr: &Attribute, struct_ident: &Ident) -> syn::Result<Self> {
        let mut instructions: Vec<DefaultInstruction> = vec![];
        if let Ok(syn::Meta::List(ref meta_list)) = attr.parse_meta() {
            for nested_meta in meta_list.nested.iter() {
                instructions.push(DefaultInstruction::parse(nested_meta, struct_ident)?)
            }
        } else {
            return Err(syn_error(
                "Invalid format for `default_instructions` attribute",
            ));
        };
        Ok(DefaultInstructionParser { instructions })
    }
}

fn syn_error(msg: &str) -> syn::Error {
    syn::Error::new_spanned("default_instructions", msg)
}

/// Converts a `DefaultInstruction` into the function tokens.
impl From<&DefaultInstruction> for TokenStream {
    fn from(value: &DefaultInstruction) -> Self {
        let (instruction_ident, param_ident, ty_tokens) = match value {
            DefaultInstruction::Create {
                struct_ident,
                instruction,
            } => (
                Ident::new(&instruction.to_snake(), Span::call_site()),
                Ident::new(&struct_ident.to_string().to_snake(), Span::call_site()),
                quote! { Create<'a, Record<'a, #struct_ident>> },
            ),
            DefaultInstruction::Delete {
                struct_ident,
                instruction,
            } => (
                Ident::new(&instruction.to_snake(), Span::call_site()),
                Ident::new(&struct_ident.to_string().to_snake(), Span::call_site()),
                quote! { Mut<Record<'a, #struct_ident>> },
            ),
            DefaultInstruction::Update {
                struct_ident,
                instruction,
            } => (
                Ident::new(&instruction.to_snake(), Span::call_site()),
                Ident::new(&struct_ident.to_string().to_snake(), Span::call_site()),
                quote! { Mut<Record<'a, #struct_ident>> },
            ),
        };
        // TODO: Have to generate the `create`, `delete` or `update` fns for specific
        // structs.
        quote! {
            fn #instruction_ident<'a>(mut #param_ident: #ty_tokens) -> ProgramResult {
                Ok(())
            }
        }
    }
}

/// Converts a `DefaultInstruction` into the actual function.
impl From<&DefaultInstruction> for ItemFn {
    fn from(value: &DefaultInstruction) -> Self {
        syn::parse2::<ItemFn>(value.into()).expect("Default instruction generation failed.")
    }
}

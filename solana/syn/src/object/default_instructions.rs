use syn::{Attribute, NestedMeta};

/// Possible default instructions for records.
#[derive(Clone, Debug)]
pub enum DefaultInstruction {
    Create(String),
    Delete(String),
    Update(String),
}

impl DefaultInstruction {
    pub fn parse(nested_meta: &NestedMeta, struct_name: &str) -> syn::Result<Self> {
        if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = nested_meta {
            let variant_string = path.get_ident().unwrap().to_string();
            if variant_string.eq("Create") {
                return Ok(DefaultInstruction::Create(struct_name.to_string()));
            } else if variant_string.eq("Delete") {
                return Ok(DefaultInstruction::Delete(struct_name.to_string()));
            } else if variant_string.eq("Update") {
                return Ok(DefaultInstruction::Update(struct_name.to_string()));
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
    pub fn parse(attr: &Attribute, struct_name: &str) -> syn::Result<Self> {
        let mut instructions: Vec<DefaultInstruction> = vec![];
        if let Ok(syn::Meta::List(ref meta_list)) = attr.parse_meta() {
            for nested_meta in meta_list.nested.iter() {
                instructions.push(DefaultInstruction::parse(nested_meta, struct_name)?)
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

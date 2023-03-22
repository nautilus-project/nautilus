pub struct NautilusAccountFieldAttributes {
    pub is_primary_key: bool,
    pub autoincrement_enabled: bool,
    pub is_authority: bool,
}

pub fn parse_field_attributes(field: &syn::Field) -> NautilusAccountFieldAttributes {
    let mut is_primary_key = false;
    let mut autoincrement_enabled = true;
    let mut is_authority = false;
    for attr in field.attrs.iter() {
        if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("primary_key") {
                is_primary_key = true;
                for nested_meta in &meta_list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta_name_value)) =
                        nested_meta
                    {
                        if meta_name_value.path.is_ident("autoincrement") {
                            if let syn::Lit::Bool(lit_bool) = &meta_name_value.lit {
                                autoincrement_enabled = lit_bool.value();
                            }
                        }
                    }
                }
            }
        } else if attr.path.is_ident("primary_key") {
            is_primary_key = true;
        } else if attr.path.is_ident("authority") {
            is_authority = true;
        }
    }
    NautilusAccountFieldAttributes {
        is_primary_key,
        autoincrement_enabled,
        is_authority,
    }
}

pub enum DefaultInstructions {
    Create,
    Delete,
    Update,
}

pub fn parse_top_level_attributes(
    struct_name: &str,
    attrs: &Vec<syn::Attribute>,
) -> Vec<nautilus_idl::IdlAccountNautilusDefaultInstructionType> {
    let mut default_instructions = Vec::new();

    for attr in attrs.iter() {
        if let Ok(syn::Meta::List(ref meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("default_instructions") {
                for nested_meta in meta_list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = nested_meta {
                        let variant_string = path.get_ident().unwrap().to_string();
                        if variant_string.eq("Create") {
                            default_instructions.push(
                                nautilus_idl::IdlAccountNautilusDefaultInstructionType::Create(
                                    struct_name.to_string(),
                                ),
                            );
                        } else if variant_string.eq("Delete") {
                            default_instructions.push(
                                nautilus_idl::IdlAccountNautilusDefaultInstructionType::Delete(
                                    struct_name.to_string(),
                                ),
                            );
                        } else if variant_string.eq("Update") {
                            default_instructions.push(
                                nautilus_idl::IdlAccountNautilusDefaultInstructionType::Update(
                                    struct_name.to_string(),
                                ),
                            );
                        } else {
                            panic!("Unknown default instruction: {}", variant_string);
                        }
                    } else {
                        panic!("Invalid format for `default_instructions` attribute");
                    }
                }
            }
        }
    }

    default_instructions
}

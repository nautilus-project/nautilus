pub struct NautilusAccountFieldAttributes {
    pub primary_key: bool,
    pub autoincrement: bool,
    pub authority: bool,
}

pub fn parse_field_attributes(field: &syn::Field) -> NautilusAccountFieldAttributes {
    let mut primary_key = false;
    let mut autoincrement = true;
    let mut authority = false;
    for attr in field.attrs.iter() {
        if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("primary_key") {
                // TODO: Add type check on Primary Key
                primary_key = true;
                for nested_meta in &meta_list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta_name_value)) =
                        nested_meta
                    {
                        if meta_name_value.path.is_ident("autoincrement") {
                            if let syn::Lit::Bool(lit_bool) = &meta_name_value.lit {
                                autoincrement = lit_bool.value();
                            }
                        }
                    }
                }
            }
        } else if attr.path.is_ident("authority") {
            authority = true;
        }
    }
    NautilusAccountFieldAttributes {
        primary_key,
        autoincrement,
        authority,
    }
}

pub fn type_to_string(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

pub fn name_to_ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::call_site())
}

pub fn name_to_ident_snake(name: &str) -> syn::Ident {
    use case::CaseExt;

    syn::Ident::new(
        &(name.to_string().to_snake()),
        proc_macro2::Span::call_site(),
    )
}

pub fn self_account_ident(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(
        &(ident.to_string() + "_self_account"),
        proc_macro2::Span::call_site(),
    )
}

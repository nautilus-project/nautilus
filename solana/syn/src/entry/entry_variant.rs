#[derive(Debug)]
pub struct NautilusEntrypointEnumVariant {
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
        discriminant: u8,
        function: syn::ItemFn,
        nautilus_structs: Vec<(String, Vec<super::required_account::RequiredAccount>)>,
    ) -> Self {
        use convert_case::{Case, Casing};
        let name = syn::Ident::new(
            &function.sig.ident.to_string().to_case(Case::Pascal),
            proc_macro2::Span::call_site(),
        );
        let (required_accounts, nautilus_objects, args) =
            parse_args(function.sig.inputs, nautilus_structs);
        Self {
            name,
            required_accounts,
            nautilus_objects,
            args,
            discriminant,
        }
    }
}

/// Parses the parameters of the user's defined function and extracts all necessary data for:
///     * Building the NautilusEntrypointEnum
///         * Instantiating any declared Nautilus objects (ie. Wallet, Token)
///         * Resolving required accounts
///     * Extracting an IDL fragment
fn parse_args(
    fn_args: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    nautilus_structs: Vec<(String, Vec<super::required_account::RequiredAccount>)>,
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
                            for (s, r) in nautilus_structs.iter() {
                                if s.contains(&ty_ident.to_string()) {
                                    nautilus_objects.push((ident.ident.clone(), ty, r.clone()));
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
    let required_accounts = super::required_account::RequiredAccount::condense_required_accounts(
        nautilus_objects
            .clone()
            .into_iter()
            .map(|(_, _, r)| r)
            .collect(),
    );
    (required_accounts, nautilus_objects, args)
}

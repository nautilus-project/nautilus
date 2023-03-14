//
//
// ----------------------------------------------------------------
//                 Nautilus object token generation
// ----------------------------------------------------------------
//
//     proc_macro2::TokenStream -> ItemStruct
//         -> NautilusObject -> * -> proc_macro2::TokenStream
//                                  * New tokens created here
//
//

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusObject {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub struct_token: syn::token::Struct,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: syn::Fields,
    pub semi_token: Option<syn::token::Semi>,
}

impl From<syn::ItemStruct> for NautilusObject {
    fn from(value: syn::ItemStruct) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            struct_token: value.struct_token,
            ident: value.ident,
            generics: value.generics,
            fields: value.fields,
            semi_token: value.semi_token,
        }
    }
}

impl syn::parse::Parse for NautilusObject {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(syn::ItemStruct::parse(input)?.into())
    }
}

impl quote::ToTokens for NautilusObject {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&NautilusObject> for proc_macro2::TokenStream {
    fn from(_ast: &NautilusObject) -> Self {
        // TODO
        quote::quote!().into()
    }
}

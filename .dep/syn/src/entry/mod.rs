//
//
// ----------------------------------------------------------------
//                 Nautilus entrypoint token generation
// ----------------------------------------------------------------
//
//     proc_macro2::TokenStream -> DataEnum
//         -> NautilusEntrypointEnum -> * -> proc_macro2::TokenStream
//                                     * New tokens created here
//
//
mod borsh;
mod processor;
mod spawn;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusEntrypointEnum {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::Data,
}

impl syn::parse::Parse for NautilusEntrypointEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse::<syn::Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![enum]) {
            let enum_token = input.parse::<syn::Token![enum]>()?;
            let ident = input.parse::<syn::Ident>()?;
            let generics = input.parse::<syn::Generics>()?;
            let (where_clause, brace, variants) = data_enum(input)?;
            Ok(syn::DeriveInput {
                attrs,
                vis,
                ident,
                generics: syn::Generics {
                    where_clause,
                    ..generics
                },
                data: syn::Data::Enum(syn::DataEnum {
                    enum_token,
                    brace_token: brace,
                    variants,
                }),
            }
            .into())
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn data_enum(
    input: syn::parse::ParseStream,
) -> syn::Result<(
    Option<syn::WhereClause>,
    syn::token::Brace,
    syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
)> {
    let where_clause = input.parse()?;

    let content;
    let brace = syn::braced!(content in input);
    let variants = content.parse_terminated(<syn::Variant as syn::parse::Parse>::parse)?;

    Ok((where_clause, brace, variants))
}

impl From<syn::DeriveInput> for NautilusEntrypointEnum {
    fn from(value: syn::DeriveInput) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            ident: value.ident,
            generics: value.generics,
            data: value.data,
        }
    }
}

impl quote::ToTokens for NautilusEntrypointEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

//
//
// Code generation
//
//
impl From<&NautilusEntrypointEnum> for proc_macro2::TokenStream {
    fn from(ast: &NautilusEntrypointEnum) -> Self {
        spawn::SpawnNautilusEntrypoint::from_ast(ast).generate()
    }
}

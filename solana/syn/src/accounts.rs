use quote::{
    ToTokens,
    quote,
};
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{
    Attribute,
    Data,
    DataStruct,
    DeriveInput,
    Field,
    Fields,
    FieldsNamed,
    Ident,
    Generics,
    Path,
    Result,
    Token,
    Visibility,
    WhereClause, 
    parse::{ Parse, ParseStream }, 
    token,
};

// The NautilusAccountStruct is the syn struct we'll use to implement parsing & token stream methods
//
// Basically, it's used to facilitate parsing of the struct tokens read in by the derive macro,
//      and turn them into the necessary tokens to implement all of the NautilusCrud traits
//
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusAccountStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}

// Parses the struct tokens and creates the NautilusAccountStruct struct
//
impl Parse for NautilusAccountStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            let struct_token = input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields, semi) = data_struct(input)?;
            Ok(DeriveInput {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: Data::Struct(DataStruct {
                    struct_token,
                    fields,
                    semi_token: semi,
                }),
            }.into())
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn data_struct(
    input: ParseStream,
) -> Result<(Option<WhereClause>, Fields, Option<Token![;]>)> {
    let mut lookahead = input.lookahead1();
    let mut where_clause = None;
    if lookahead.peek(Token![where]) {
        where_clause = Some(input.parse()?);
        lookahead = input.lookahead1();
    }

    if where_clause.is_none() && lookahead.peek(token::Paren) {
        let fields = input.parse()?;

        lookahead = input.lookahead1();
        if lookahead.peek(Token![where]) {
            where_clause = Some(input.parse()?);
            lookahead = input.lookahead1();
        }

        if lookahead.peek(Token![;]) {
            let semi = input.parse()?;
            Ok((where_clause, Fields::Unnamed(fields), Some(semi)))
        } else {
            Err(lookahead.error())
        }
    } else if lookahead.peek(token::Brace) {
        let fields = input.parse()?;
        Ok((where_clause, Fields::Named(fields), None))
    } else if lookahead.peek(Token![;]) {
        let semi = input.parse()?;
        Ok((where_clause, Fields::Unit, Some(semi)))
    } else {
        Err(lookahead.error()) // TODO: Can only be for structs
    }
}

// Allows us to, in the above parsing method, convert the parsed tokens into the NautilusAccountStruct
//
impl From<DeriveInput> for NautilusAccountStruct {
    fn from(value: DeriveInput) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            ident: value.ident,
            generics: value.generics,
            data: value.data,
        }
    }
}

// Allows us to convert the input into a token stream from the parsed NautilusAccountStruct
//      using the below business logic
//
impl ToTokens for NautilusAccountStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

// Allows us to convert the built-out NautilusAccountStruct struct into the tokens we need
//
// Basically, this is where all the magic happens
//
impl From<&NautilusAccountStruct> for TokenStream {
    fn from(ast: &NautilusAccountStruct) -> Self {
        
        let name = &ast.ident;
        let attrs = &ast.attrs;
        let fields = if let Data::Struct(
            DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }
        ) = &ast.data {
            named
        } else {
            unimplemented!() // TODO: Can only be for structs
        };

        fn is_primary_key(field: &Field) -> bool {
            if field.attrs.len() == 0 { return false };
            for attr in field.attrs.iter() {
                for seg in attr.path.segments.iter() {
                    if seg.ident == "primary_key" { return true }
                };
            }
            false
        }

        fn is_authority(field: &Field) -> bool {
            if field.attrs.len() == 0 { return false };
            for attr in field.attrs.iter() {
                for seg in attr.path.segments.iter() {
                    if seg.ident == "authority" { return true }
                };
            }
            false
        }

        // let optionized = fields.iter().map(|f| {
        //     let original_ty = f.ty.clone();
        //     let ty = Path
        //     Field {
        //         attrs: Vec::new(),
        //         vis: Visibility::Inherited,
        //         ident: f.ident,
        //         colon_token: f.colon_token,
        //         ty,
        //     }
        // });

        println!("  -- Name: {}", name);
        println!("  -- Attrs: {:?}", attrs);
        println!("  -- Attrs len: {}", attrs.len());

        println!("  -- Fields: {:?}", fields);
        println!("  -- Field Attrs: {:?}", fields.first().unwrap().attrs);
        println!("  -- Is Primary Key: {}", is_primary_key(fields.first().unwrap()));
        println!("  -- Is Authority: {}", is_authority(fields.first().unwrap()));
        
        quote! {
            
            impl BorshDeserialize for #name {
                fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                    BorshDeserialize::deserialize(&mut &buf[..])
                }
            }
    
            impl BorshSerialize for #name {
                fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    BorshSerialize::serialize(self, writer)
                }
            }
    
            impl NautilusAccountBorsh for #name {}

            impl NautilusAccountCreate for #name {}

            impl NautilusAccountDelete for #name {}

        }.into()
    }
}
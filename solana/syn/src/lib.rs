// use proc_macro::TokenStream;
// use syn::ItemStruct;
// use syn::parse::{Error as ParseError, Parse, ParseStream, Result as ParseResult};
// use syn::punctuated::Punctuated;
// use syn::spanned::Spanned;
// use syn::token::Comma;
// use syn::{bracketed, Expr, Ident, LitStr, Token};
// use quote::quote;
// use syn::parse_macro_input;

// pub fn parse(input: TokenStream) -> TokenStream {
//     // let ast: ItemStruct = syn::parse(input).unwrap();
//     let ast = parse_macro_input!(input as ItemStruct);

//     let struct_name = ast.ident;
//     let table_name = struct_name.to_string().to_lowercase() + "_table";
//     println!("  * Struct name: {}", &struct_name.to_string());
//     println!("  Fields:");
//     for field in ast.fields {
//         println!(
//             "  -- {} : -type-", 
//             &field.ident.expect("Not Found").to_string()
//         );
//         println!("  -- {:#?}", field.ty);
//     }
    
//     let att = &ast.attrs[0];
//     println!("  * Attribute name: {}", att.path.segments[0].ident.to_string());
//     println!("  Args:");
//     println!("{}", &att.tokens);

//     let output = quote! {

//         impl #struct_name {
//             fn new_inner(
//                 id: u32,
//                 name: String,
//             ) -> #struct_name {
                
//                 #struct_name {
//                     id,
//                     name,
//                 }
//             }
        
//             fn update_inner(
//                 &mut self,
//                 id: Option<u32>,
//                 name: Option<String>,
//             ) {
                
//                 match id {
//                     Some(id) => self.id = id,
//                     None => (),
//                 }
//                 match name {
//                     Some(name) => self.name = name,
//                     None => (),
//                 }
//             }
//         }

//         impl NautilusCrud for #struct_name {

//             type NautilusCrudObject = #struct_name;
        
//             const TABLE_NAME: &'static str = #table_name;
//             const PRIMARY_KEY: &'static str = "id";
//             const AUTO_INCREMENT: bool = true;
        
//             fn test_new_inner() -> Self {
//                 #struct_name {
//                     id: 1,
//                     name: String::from("Joe C"),
//                 }
//             }
//         }
//     };
//     println!("{}", output);

//     TokenStream::from(output)
// }

// #[derive(Debug)]
// pub struct NautilusCrudStruct {
//     // Name of the accounts struct.
//     pub ident: Ident,
//     // Generics + lifetimes on the accounts struct.
//     pub generics: Generics,
//     // Fields on the accounts struct.
//     pub fields: Vec<NautilusCrudField>,
//     // Instruction data api expression.
//     instruction_api: Option<Punctuated<Expr, Comma>>,
// }

// impl Parse for NautilusCrudStruct {
//     fn parse(input: ParseStream) -> ParseResult<Self> {
//         let strct = <ItemStruct as Parse>::parse(input)?;
//         accounts_parser::parse(&strct)
//     }
// }

// impl From<&NautilusCrudStruct> for TokenStream {
//     fn from(accounts: &NautilusCrudStruct) -> Self {
//         accounts_codegen::generate(accounts)
//     }
// }

// impl ToTokens for NautilusCrudStruct {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         tokens.extend::<TokenStream>(self.into());
//     }
// }

// impl NautilusCrudStruct {
//     pub fn new(
//         strct: ItemStruct,
//         fields: Vec<NautilusCrudField>,
//         instruction_api: Option<Punctuated<Expr, Comma>>,
//     ) -> Self {
//         let ident = strct.ident.clone();
//         let generics = strct.generics;
//         Self {
//             ident,
//             generics,
//             fields,
//             instruction_api,
//         }
//     }

//     // Return value maps instruction name to type.
//     // E.g. if we have `#[instruction(data: u64)]` then returns
//     // { "data": "u64"}.
//     pub fn instruction_args(&self) -> Option<HashMap<String, String>> {
//         self.instruction_api.as_ref().map(|instruction_api| {
//             instruction_api
//                 .iter()
//                 .map(|expr| {
//                     let arg = parser::tts_to_string(expr);
//                     let components: Vec<&str> = arg.split(" : ").collect();
//                     assert!(components.len() == 2);
//                     (components[0].to_string(), components[1].to_string())
//                 })
//                 .collect()
//         })
//     }

//     pub fn field_names(&self) -> Vec<String> {
//         self.fields
//             .iter()
//             .map(|field| field.ident().to_string())
//             .collect()
//     }
// }

// // #[allow(clippy::large_enum_variant)]
// // #[derive(Debug)]
// // pub enum NautilusCrudField {
// //     Field(Field),
// //     CompositeField(CompositeField),
// // }

// // impl NautilusCrudField {
// //     fn ident(&self) -> &Ident {
// //         match self {
// //             NautilusCrudField::Field(field) => &field.ident,
// //             NautilusCrudField::CompositeField(c_field) => &c_field.ident,
// //         }
// //     }

// //     pub fn ty_name(&self) -> Option<String> {
// //         let qualified_ty_name = match self {
// //             NautilusCrudField::Field(field) => match &field.ty {
// //                 Ty::NautilusCrud(account) => Some(parser::tts_to_string(&account.account_type_path)),
// //                 Ty::ProgramNautilusCrud(account) => {
// //                     Some(parser::tts_to_string(&account.account_type_path))
// //                 }
// //                 _ => None,
// //             },
// //             NautilusCrudField::CompositeField(field) => Some(field.symbol.clone()),
// //         };

// //         qualified_ty_name.map(|name| match name.rsplit_once(" :: ") {
// //             Some((_prefix, suffix)) => suffix.to_string(),
// //             None => name,
// //         })
// //     }
// // }
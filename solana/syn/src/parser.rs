// use proc_macro2::Span;
use std::io::Result;
use syn::{
    Fields,
    Ident,
    ItemEnum,
    ItemStruct,
    Variant,
    punctuated::{ Punctuated },
    token::{ Comma },
};

/**
 * Here we're going to just get a token stream of the enum marked as the
 *      entrypoint (NautilusEntrypoint).
 * 
 * Our required return data:
 *      * enum_name
 *      * enum_variants
 */
pub fn parse_entrypoint(input: ItemEnum) -> Result<(Ident, Punctuated<Variant, Comma>)> {

    let enum_name = input.ident;
    let enum_variants = input.variants;
    Ok((enum_name, enum_variants))
}

/**
 * Here we're going to just get a token stream of the struct marked as
 *      a CRUD object (Nautilus).
 * 
 * Our required return data:
 *      * struct_name
 *      * struct_fields
 *      * primary_key_field
 *      * autoincrement
 */
pub fn parse_state(input: ItemStruct) -> Result<(Ident, Fields, Ident, Ident)> {

    // TODO: Parse out state
    //  (Current)
    //
    let struct_name = input.ident;
    let struct_fields = input.fields;
    let primary_key_field = struct_name.clone();
    let autoincrement = struct_name.clone();
    Ok((struct_name, struct_fields, primary_key_field, autoincrement))
}
use proc_macro2::{ Span, TokenStream };
use quote::quote;
use syn::{
    Ident,
    ItemEnum,
    ItemStruct,
    Variant,
    punctuated::{ Punctuated },
    token::{ Comma },
};

/**
 * Create a copy of the enum where we populate the arg structs.
 * 
 * We do this for now as a redundant step that saves the user code,
 *      but soon we can use user-provided arg structs as override commands
 *      for any default Nautilus CRUD operation.
 * 
 *      enum MyEnum {
 *          CreatePerson,
 *          UpdatePerson,
 *      }
 *      enum NautilusEnumCopy {
 *          CreatePerson(CreatePersonArgs),
 *          UpdatePerson(UpdatePersonArgs),
 *      }
 * 
 * Then we use the NautilusEnumCopy to actually define our program's processor.
 */
pub fn build_nautilus_enum(
    input: ItemEnum, 
    enum_name: Ident, 
    enum_variants: Punctuated<Variant, Comma>,
) -> (Ident, Punctuated<Variant, Comma>, ItemEnum) {
    // TODO: Regex parsing steps
    let mut nautilus_enum = input.clone();
    nautilus_enum.ident = Ident::new(&(enum_name.to_string() + "Nautilus"), Span::call_site());
    // for x in nautilus_enum.variants.iter_mut() {
    //     x = Ident::new(&(x.ident.to_string() + "Nautilus"));
    // }
    let nautilus_enum_name = nautilus_enum.ident.clone();
    let nautilus_enum_variants = nautilus_enum.variants.clone();
    println!("NAUTILUS ENUM NAME: {}", nautilus_enum_name);
    println!("NAUTILUS ENUM VARIANTS_LENGTH: {}", nautilus_enum_variants.len());
    println!("NAUTILUS ENUM VARIANTS: {:#?}", nautilus_enum_variants);
    (
        nautilus_enum_name,
        nautilus_enum_variants,
        nautilus_enum,
    )
}

/**
 * Building processor for the NautilusEnumCopy enum clone.
 * 
 *      match instruction {
 *          NautilusEnumCopy::Variant(args) => AssociatedStruct::derived_fn(program_id, accounts, args)
 *      }
 */
pub fn build_processor(
    input: ItemEnum, 
    enum_name: Ident, 
    enum_variants: Punctuated<Variant, Comma>,
) -> Ident {
    // TODO: Build processor from variants
    // let processor = proc_macro2::TokenStream::new();
    // for (variant, struct_name) in enum_variants {
    //     let fn_name = variant.to_case(Case::Snake);
    //     processor.extend(quote! {
    //         #enum_name::#variant => #struct_name::#fn_name,
    //     })
    // };
    // processor
    // quote! {
    //     #processor
    // }
    Ident::new("demo", Span::call_site())
}

/**
 * Here's where we actually build the entrypoint function under our user's 
 *      original enum, to enable the `MyEnum::entrypoint` call.
 */
pub fn impl_entrypoint(
    input: ItemEnum, 
    enum_name: Ident, 
    enum_variants: Punctuated<Variant, Comma>, 
) -> TokenStream {
    
    let (
        nautilus_enum_name,
        nautilus_enum_variants,
        nautilus_enum_tokens, 
    ) = build_nautilus_enum(input.clone(), enum_name.clone(), enum_variants);
    // let processor = build_processor(nautilus_enum_name.clone(), nautilus_enum_variants);
    // quote! {

    //     solana_program::entrypoint!(process_instruction)

    //     #nautilus_enum_tokens

    //     fn process_instruction(
    //         program_id: &Pubkey,
    //         accounts: &[AccountInfo],
    //         input: &[u8],
    //     ) -> ProgramResult {

    //         let instruction = #nautilus_enum_name::try_from_slice(&input)?;
    //         match instruction {
    //             #processor
    //         }
    //         Ok(())
    //     }
    // }
    TokenStream::new()
}
use convert_case::{ Case, Casing };
use proc_macro2::{ Span, TokenStream };
use quote::quote;
use syn::{
    Ident,
    ItemEnum,
    Variant,
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
pub fn build_nautilus_enum(input: ItemEnum) -> ItemEnum {
    let mut nautilus_enum = input.clone();
    nautilus_enum.ident = Ident::new(&(nautilus_enum.ident.to_string() + "Nautilus"), Span::call_site());
    nautilus_enum.variants.iter_mut().for_each(|x| {
        x.ident = Ident::new(&(x.ident.to_string() + "Nautilus"), Span::call_site());
    });
    nautilus_enum
}

/**
 * Building processor for the NautilusEnumCopy enum clone.
 * 
 *      match instruction {
 *          NautilusEnumCopy::Variant(args) => AssociatedStruct::derived_fn(program_id, accounts, args)
 *      }
 */
pub fn build_processor(input: ItemEnum) -> TokenStream {
    let enum_ident = input.ident;
    let mut processor = TokenStream::new(); 
    input.variants.iter().for_each(|x| {
        let mut struct_token = x.ident.to_string().to_case(Case::Title);
        let mut struct_token_vec: Vec<&str> = struct_token.split(" ").collect();
        let fn_prefix = struct_token_vec.get(0).expect("Empty vector for enum variant.").to_string();
        struct_token_vec.remove(0);
        struct_token_vec.remove(struct_token_vec.len() - 1);
        let mut struct_name = "".to_string();
        let mut fn_name = fn_prefix;
        struct_token_vec.iter().for_each(|s| {
            struct_name += s;
            fn_name += s;
        });
        fn_name = fn_name.to_case(Case::Snake);
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let fn_ident = Ident::new(&fn_name, Span::call_site());
        processor.extend(
            quote! {
                #enum_ident::#x => #struct_ident::#fn_ident (),
            }
        )
    });
    processor
}

/**
 * Here's where we actually build the entrypoint function under our user's 
 *      original enum, to enable the `MyEnum::entrypoint` call.
 */
pub fn impl_entrypoint(input: ItemEnum) -> TokenStream {
    let nautilus_enum = build_nautilus_enum(input.clone());
    let nautilus_enum_name = nautilus_enum.ident.clone();
    let processor = build_processor(nautilus_enum.clone());
    quote! {
        use solana_program::{
            account_info::AccountInfo, 
            entrypoint, 
            entrypoint::ProgramResult, 
            pubkey::Pubkey,
        };

        #nautilus_enum;

        fn process_instruction(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            input: &[u8],
        ) -> ProgramResult {

            let instruction = #nautilus_enum_name::try_from_slice(&input)?;
            match instruction {
                #processor
            }
        }

        entrypoint!(process_instruction);
    }
}
use proc_macro2::{ Span, TokenStream };
// use quote::quote;
use syn::{
    Fields,
    Ident,
    ItemStruct,
};

pub fn build_inner(struct_name: Ident, struct_fields: Fields) -> Ident {
    // TODO: Turn into the new() and update() functions based on fields.
    // let new_inner = struct_fields;
    // let update_inner = struct_fields;
    // quote! {
    //     impl #struct_name {
    //         #new_inner
    //         #update_inner
    //     }
    // }
    Ident::new("demo", Span::call_site())
}

pub fn build_state(
    struct_name: Ident, 
    struct_fields: Fields,
    primary_key: Ident,
    autoincrement: Ident,
) -> Ident {
    // TODO: Use params to determine seeds
    // let seeds = struct_fields;
    // quote! {
    //     impl NautilusAccount for #struct_name {

    //         use solana_program::pubkey::Pubkey;

    //         fn address(&self) -> (Pubkey, u8) {
    //             Pubkey::find_program_address(
    //                 #seeds,
    //                 self.program_id,
    //             )
    //         }

    //         fn seeds(&self) -> &[&[u8]] {
    //             #seeds
    //         }
    //     }
    // }
    Ident::new("demo", Span::call_site())
}

pub fn build_allocate_fields(struct_fields: Fields) -> (Ident, Ident, Ident) {
    // TODO: Build allocate fields
    (
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
    )
}

pub fn build_create_fields(struct_fields: Fields) -> (Ident, Ident, Ident) {
    // TODO: Build create fields
    (
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
    )
}

pub fn build_update_fields(struct_fields: Fields) -> (Ident, Ident, Ident) {
    // TODO: Build update fields
    (
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
    )
}

pub fn build_delete_fields(struct_fields: Fields) -> (Ident, Ident, Ident) {
    // TODO: Build delete fields
    (
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
        Ident::new("demo", Span::call_site()),
    )
}

pub fn build_crud(struct_name: Ident, struct_fields: Fields) -> Ident {
    // let struct_name_lower = &struct_name.to_string().to_lowercase();
    // let (
    //     allocate_args_name, 
    //     allocate_args_fields, 
    //     allocate_args_param_fields
    // ) = build_allocate_fields(struct_fields);
    // let (
    //     create_args_name, 
    //     create_args_fields, 
    //     create_args_param_fields
    // ) = build_create_fields(struct_fields);
    // let (
    //     update_args_name, 
    //     update_args_fields, 
    //     update_args_param_fields
    // ) = build_update_fields(struct_fields);
    // let (
    //     delete_args_name, 
    //     delete_args_fields, 
    //     delete_args_param_fields
    // ) = build_delete_fields(struct_fields);
    // quote! {
    //     impl NautilusAllocate for #struct_name {
            
    //         pub struct #allocate_args_name {
    //             #allocate_args_fields
    //         }

    //         pub fn allocate #struct_name_lower (
    //             program_id: &Pubkey,
    //             accounts: $[AccountInfo],
    //             args: #allocate_args_name,
    //         ) -> ProgramResult {

    //             let accounts_iter = accounts.iter();
    //             let self_account = next_account_info()?;
    //             let payer = next_account_info()?;
    //             let system_program = next_account_info()?;

    //             let data = #struct_name ::new(
    //                 #allocate_args_param_fields
    //             );

    //             data.validate_pda(&self_account.key)?;

    //             data.allocate(self_account, payer, system_program)
    //         }
    //     }

    //     impl NautilusCreate for #struct_name {

    //         pub struct #create_args_name {
    //             #create_args_fields
    //         }

    //         pub fn create #struct_name_lower (
    //             program_id: &Pubkey,
    //             accounts: $[AccountInfo],
    //             args: #create_args_name,
    //         ) -> ProgramResult {

    //             let accounts_iter = accounts.iter();
    //             let self_account = next_account_info()?;
    //             let payer = next_account_info()?;
    //             let system_program = next_account_info()?;

    //             let data = #struct_name ::new(
    //                 #create_args_param_fields
    //             );

    //             data.validate_pda(&self_account.key)?;

    //             data.create(self_account, payer, system_program)
    //         }
    //     }

    //     impl NautilusUpdate for #struct_name {

    //         pub struct update_args_name {
    //             #update_args_fields
    //         }

    //         pub fn update #struct_name_lower (
    //             program_id: &Pubkey,
    //             accounts: $[AccountInfo],
    //             args: #update_args_name,
    //         ) -> ProgramResult {

    //             let accounts_iter = accounts.iter();
    //             let self_account = next_account_info()?;
    //             let payer = next_account_info()?;
    //             let system_program = next_account_info()?;

    //             let data = #struct_name ::new(
    //                 #update_args_param_fields
    //             );

    //             data.validate_pda(&self_account.key)?;

    //             data.update(self_account, payer, system_program)
    //         }
    //     }

    //     impl NautilusDelete for #struct_name {

    //         pub struct #delete_args_name {
    //             #delete_args_fields
    //         }

    //         pub fn delete #struct_name_lower (
    //             program_id: &Pubkey,
    //             accounts: $[AccountInfo],
    //             args: #delete_args_name,
    //         ) -> ProgramResult {

    //             let accounts_iter = accounts.iter();
    //             let self_account = next_account_info()?;
    //             let payer = next_account_info()?;
    //             let system_program = next_account_info()?;

    //             let data = #struct_name ::new(
    //                 #delete_args_param_fields
    //             );

    //             data.validate_pda(&self_account.key)?;

    //             data.delete(self_account, payer, system_program)
    //         }
    //     }
    // }
    Ident::new("demo", Span::call_site())
}

/**
 * Here's where we implement all the necessary functions for the state struct.
 * 
 * We lay everything down, and the entrypoint macro will determine which get used.
 * 
 *  Inner (ex.):
 *      * new(id: u32, name: String) -> Self
 *      * update(id: Option<u32>, name: Option<String>) -> Self
 * 
 *  State (ex.):
 *      * validate_pda(&self, pubkey: Pubkey) -> bool/assert
 *      * address(&self) -> (Pubkey, u8)
 *      * seeds(&self) -> &[&[u8]]
 *      * seeds_and_bump(&self) -> &[&[u8]]
 * 
 *  Crud (ex.):
 *      * create(self_account, payer, system_program) -> ProgramResult
 */
pub fn impl_state(
    input: ItemStruct,
    struct_name: Ident, 
    struct_fields: Fields, 
    primary_key_field: Ident, 
    autoincrement: Ident,
) -> TokenStream {

    let inner = build_inner(struct_name.clone(), struct_fields.clone());
    // let state = build_state(struct_name.clone(), struct_fields.clone(), primary_key_field, autoincrement);
    // let crud = build_crud(struct_name, struct_fields);
    // quote! {
    //     #inner
    //     #state
    //     #crud
    // }
    TokenStream::new()
}
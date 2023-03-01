pub fn nautilus_create_tokens(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountCreate for #struct_name {}
    }
}

pub fn nautilus_delete_tokens(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountDelete for #struct_name {}
    }
}

pub fn nautilus_update_tokens(
    struct_name: &syn::Ident,
    struct_name_optionized: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountUpdate for #struct_name {

            fn process_nautilus_update_data<#struct_name_optionized>(
                &mut self,
                update_data: #struct_name_optionized,
            ) {

                todo!()
            }
        }
    }
}

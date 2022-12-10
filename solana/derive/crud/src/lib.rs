#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

#[proc_macro_derive(Nautilus, attributes(nautilus))]
pub fn derive_nautilus_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = input.ident;
    let table_name = struct_name.to_string().to_lowercase() + "_table";

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_names: Vec<syn::Ident> = fields.iter().map(|field| field.ident.clone().unwrap()).collect();
    let field_types: Vec<syn::Type> = fields.iter().map(|field| field.ty.clone()).collect();
    fn map_match_statement(ident: syn::Ident) -> proc_macro2::TokenStream {
        quote! {
            match #ident {
                Some(#ident) => self.#ident = #ident,
                None => (),
            }
        }
    }
    let field_type_match_statements: Vec<proc_macro2::TokenStream> = fields.iter()
        .map(|field| 
            map_match_statement(field.ident.clone().unwrap())
        ).collect();
        
    if input.attrs.is_empty() || input.attrs[0].path.segments[0].ident.to_string() != "nautilus" {
        panic!("You need to provide the #[nautilus] attribute below #[derive].");
    };
    if input.attrs[0].tokens.is_empty() {
        panic!("You need to provide an arg for 'primary_key' in the #[nautilus] attribute.");
    };

    // --- This junk can go
    let att = &input.attrs[0];
    let att_segments = &att.path.segments;
    let mut nautilus_invoked = false;
    for seg in att_segments {
        let attribute_name = seg.ident.to_string();
        println!("  * Attribute name: {}", &attribute_name);
        if attribute_name == "nautilus" {
            println!(" -- nautilus invoked");
            nautilus_invoked = true;
        };
    };
    if !nautilus_invoked {
        panic!("You need to provide the #[nautilus] attribute below #[derive].");
    };
    println!("  Args:");
    println!("{:?}", &att.tokens);
    //

    let mut create = proc_macro2::TokenStream::new();
    let mut update = proc_macro2::TokenStream::new();
    let mut delete = proc_macro2::TokenStream::new();
    // Here's where we want to place the parsing of the args/attributes
    if true {
        create = quote! { 
            use nautilus_solana::crud::NautilusCreate;
            impl NautilusCreate for #struct_name {} 
        };
    }
    if true {
        update = quote! { 
            use nautilus_solana::crud::NautilusUpdate;
            impl NautilusUpdate for #struct_name {} 
        };
    }
    if true {
        delete = quote! { 
            use nautilus_solana::crud::NautilusDelete;
            impl NautilusDelete for #struct_name {} 
        };
    }

    let output = quote! {

        impl #struct_name {
            fn new_inner(
                #(#field_names: #field_types,)*
            ) -> #struct_name {
                
                #struct_name {
                    #(#field_names,)*
                }
            }
        
            fn update_inner(
                &mut self,
                #(#field_names: Option<#field_types>,)*
            ) {
                
                #(#field_type_match_statements)*
            }
        }

        use nautilus_solana::crud::NautilusAccount;

        impl NautilusAccount for #struct_name {

            type NautilusCrudObject = #struct_name;
        
            const TABLE_NAME: &'static str = #table_name;
            const PRIMARY_KEY: &'static str = "id";
            const AUTO_INCREMENT: bool = true;
        }
        #create
        #update
        #delete
    };
    
    println!("{}", output);
    proc_macro::TokenStream::from(output)
}


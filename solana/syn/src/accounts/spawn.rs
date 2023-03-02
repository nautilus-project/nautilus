pub struct SpawnNautilusAccount {
    autoincrement_enabled: bool,
    table_name: String,

    fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,

    ident_struct_name: syn::Ident,
    ident_optionized_struct_name: syn::Ident,
    ident_primary_key: syn::Ident,
    idents_authorities: Vec<syn::Ident>,

    optionized_struct_fields: Vec<(
        syn::Ident,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    )>,

    ty_primary_key: syn::Type,
}

impl SpawnNautilusAccount {
    pub fn from_ast(
        ast: &crate::NautilusAccountStruct,
    ) -> Result<Self, super::error::EnforceStructsError> {
        let mut primary_key_ident_opt: Option<(syn::Ident, syn::Type)> = None;
        let mut autoincrement_enabled: bool = true;
        let mut idents_authorities: Vec<syn::Ident> = vec![];

        let ident_struct_name = ast.ident.clone();
        let ident_optionized_struct_name = syn::Ident::new(
            &(ident_struct_name.to_string() + "Optionized"),
            proc_macro2::Span::call_site(),
        );

        let table_name = ident_struct_name.to_string().to_lowercase();

        let fields = if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) = ast.data
        {
            named.clone()
        } else {
            return Err(super::error::EnforceStructsError());
        };

        let mut optionized_struct_fields: Vec<(
            syn::Ident,
            proc_macro2::TokenStream,
            proc_macro2::TokenStream,
        )> = vec![];

        for f in fields.clone() {
            let parsed_attributes = super::parser::parse_field_attributes(&f);
            if !parsed_attributes.autoincrement {
                autoincrement_enabled = parsed_attributes.autoincrement;
            }
            if parsed_attributes.primary_key {
                primary_key_ident_opt = Some((f.ident.clone().unwrap(), f.ty.clone()));
            }
            if parsed_attributes.authority {
                idents_authorities.push(f.ident.clone().unwrap());
            }

            let field_name = &f.ident;
            let field_ty = &f.ty;
            optionized_struct_fields.push(match parsed_attributes.primary_key {
                true => (
                    field_name.clone().unwrap(),
                    quote::quote! { #field_ty },
                    quote::quote! { #field_name: #field_ty },
                ),
                false => (
                    field_name.clone().unwrap(),
                    quote::quote! { std::option::Option<#field_ty> },
                    quote::quote! { #field_name: std::option::Option<#field_ty> },
                ),
            });
        }

        let (ident_primary_key, ty_primary_key) = match primary_key_ident_opt {
            Some((ident, ty)) => (ident, ty),
            None => todo!("Throw an error on None value"),
        };

        Ok(SpawnNautilusAccount {
            autoincrement_enabled,
            table_name,
            fields,
            ident_struct_name,
            ident_optionized_struct_name,
            ident_primary_key,
            idents_authorities,
            optionized_struct_fields,
            ty_primary_key,
        })
    }

    fn idents_optionized_struct_fields(&self) -> Vec<syn::Ident> {
        self.optionized_struct_fields
            .iter()
            .map(|(o, _, _)| o.clone())
            .collect()
    }

    fn tys_optionized_struct_fields(&self) -> Vec<proc_macro2::TokenStream> {
        self.optionized_struct_fields
            .iter()
            .map(|(_, o, _)| o.clone())
            .collect()
    }

    fn tokens_optionized_struct_fields(&self) -> Vec<proc_macro2::TokenStream> {
        self.optionized_struct_fields
            .iter()
            .map(|(_, _, o)| o.clone())
            .collect()
    }

    pub fn generate(
        &self,
    ) -> Result<proc_macro2::TokenStream, super::error::EnforcePrimaryKeyType> {
        let tokens_primary_key_seed = super::data::build_tokens_primary_key_seed(
            &self.ident_primary_key,
            &self.ty_primary_key,
        )?;

        let optionized_nautilus_account_tokens = super::data::nautilus_optionized(
            &self.ident_optionized_struct_name,
            &self.tokens_optionized_struct_fields(),
            &self.ident_struct_name,
            &self.fields,
            &self.ident_primary_key,
        );
        let optionized_nautilus_account_borsh_tokens = super::borsh::nautilus_borsh_optionized(
            &self.ident_optionized_struct_name,
            &self.idents_optionized_struct_fields(),
            &self.tys_optionized_struct_fields(),
        );
        let optionized_nautilus_account_data_tokens = super::data::nautilus_account_data_tokens(
            &self.ident_optionized_struct_name,
            &self.table_name,
            self.autoincrement_enabled,
            tokens_primary_key_seed.clone(),
        );

        let (count_authorities, check_authorities_tokens) =
            super::data::build_count_check_authorities_tokens(&self.idents_authorities);

        let self_nautilus_account_borsh_tokens =
            super::borsh::nautilus_borsh_self(&self.ident_struct_name, &self.fields);
        let self_nautilus_account_data_tokens = super::data::nautilus_account_data_tokens(
            &self.ident_struct_name,
            &self.table_name,
            self.autoincrement_enabled,
            tokens_primary_key_seed.clone(),
        );
        let self_nautilus_account_auth_tokens = super::data::nautilus_account_auth_tokens(
            &self.ident_struct_name,
            check_authorities_tokens,
            count_authorities,
        );
        let self_nautilus_create_tokens =
            super::crud::nautilus_create_tokens(&self.ident_struct_name);
        let self_nautilus_delete_tokens =
            super::crud::nautilus_delete_tokens(&self.ident_struct_name);
        let self_nautilus_update_tokens =
            super::crud::nautilus_update_tokens(&self.ident_struct_name);

        Ok(quote::quote! {
            #optionized_nautilus_account_tokens
            #optionized_nautilus_account_borsh_tokens
            #optionized_nautilus_account_data_tokens
            #self_nautilus_account_borsh_tokens
            #self_nautilus_account_data_tokens
            #self_nautilus_account_auth_tokens
            #self_nautilus_create_tokens
            #self_nautilus_delete_tokens
            #self_nautilus_update_tokens
        }
        .into())
    }
}

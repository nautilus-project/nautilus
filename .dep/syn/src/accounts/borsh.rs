pub fn nautilus_borsh_self(
    struct_name: &syn::Ident,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let borsh_ser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: nautilus::borsh::ser::BorshSerialize }
    });
    let borsh_ser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { nautilus::borsh::BorshSerialize::serialize(&self.#field_name, writer)? }
    });
    let borsh_deser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: nautilus::borsh::de::BorshDeserialize }
    });
    let borsh_deser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { #field_name: nautilus::borsh::BorshDeserialize::deserialize(buf)? }
    });
    quote::quote! {
        impl nautilus::borsh::ser::BorshSerialize for #struct_name
        where
            #(#borsh_ser_where,)*
        {
            fn serialize<W: nautilus::borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), nautilus::borsh::maybestd::io::Error> {
                #(#borsh_ser_impl;)*
                Ok(())
            }
        }
        impl nautilus::borsh::de::BorshDeserialize for #struct_name
        where
            #(#borsh_deser_where,)*
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, nautilus::borsh::maybestd::io::Error> {
                Ok(Self {
                    #(#borsh_deser_impl,)*
                })
            }
        }
    }
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::model::Model;

pub fn derive(item: DeriveInput) -> Result<TokenStream> {
    let model = Model::from_item(&item)?;
    let struct_name = &item.ident;

    let mut from_table_impl = quote! {};

    for field in model.fields() {
        let field_name = field.ident.as_ref().unwrap();
        let alias_name = field.alias_name();

        from_table_impl = quote! {
            #from_table_impl
            #field_name: {
                let idx = param.get_field_index_by_name(#alias_name)?;
                let content = param.get_field_by_index(idx)?.get_chars()?;
                content
            },
        };
    }

    Ok(quote! {
        impl FromTable for #struct_name {
            fn from_table(param: &mut rsrfc::RfcParameter) -> Result<Self, RfcErrorInfo> {
                let mut result = Self {
                    #from_table_impl
                };

                Ok(result)
            }
        }
    })
}

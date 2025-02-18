use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

// use crate::error::Result;
use crate::model::Model;

pub fn derive(item: DeriveInput) -> Result<TokenStream> {
    let _model = Model::from_item(&item)?;
    let struct_name = &item.ident;

    let mut from_method_impl = quote! {};

    for field in _model.fields() {
        let field_name = field.ident.as_ref().unwrap();
        let alias_name = field.alias_name();

        if field.is_type("Vec") {
            let inner_type = field.inner_type().unwrap();
            let inner_type_name = quote! { #inner_type };
            from_method_impl = quote! {
                #from_method_impl
                #field_name: {
                    let param = method.get_mut_parameter(#alias_name).ok_or(RfcErrorInfo::custom("unknown field #alias_name"))?;
                    let total = param.get_row_count()?;
                    let mut data: Vec<#inner_type_name> = vec![];

                    for i in 0..total {
                        param.set_row(i)?;
                        let el = #inner_type_name::from_table(param)?;
                        data.push(el);
                    }
                    data
                },
            }
        }
    }

    Ok(quote! {
        impl FromMethod for #struct_name {
            fn from_method(method: &mut rsrfc::RfcFunction) -> Result<Self, RfcErrorInfo> {
                let mut response = Self {
                    #from_method_impl
                };

                Ok(response)
            }
        }
    })
}

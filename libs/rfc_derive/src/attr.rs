use proc_macro2::Span;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Ident, LitStr,
    Result,
};

use crate::util;

pub struct AttributeSpanWrapper<T> {
    pub item: T,
    pub attribute_span: Span,
}

pub enum FieldAttr {
    Alias(Ident, LitStr),
}

impl Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        match &*name_str {
            "alias" => Ok(FieldAttr::Alias(
                name,
                util::parse_eq(input, util::ALIAS_NOTE)?,
            )),
            _ => todo!(),
        }
    }
}

pub fn parse_attributes<T>(attrs: &[Attribute]) -> Result<Vec<AttributeSpanWrapper<T>>>
where
    T: Parse,
{
    let mut out = Vec::new();
    for attr in attrs {
        if attr.meta.path().is_ident("sap") {
            let map = attr
                .parse_args_with(Punctuated::<T, Comma>::parse_terminated)?
                .into_iter()
                .map(|a| AttributeSpanWrapper {
                    item: a,
                    attribute_span: attr.meta.span(),
                });
            out.extend(map);
        }
    }

    Ok(out)
}

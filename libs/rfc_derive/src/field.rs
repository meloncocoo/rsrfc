use proc_macro2::Span;
use syn::{
    parse::Parse, Field as SynField, GenericArgument, Ident, LitStr, PathArguments, Result, Type,
};

use crate::{
    attr::{self, AttributeSpanWrapper},
    util,
};

type Index = usize;

pub struct AliasIdent {
    alias_name: String,
    span: Span,
}

enum FieldAttr {
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

pub struct Field {
    _field: SynField,
    _index: usize,
    pub ty: Type,
    pub ident: Option<Ident>,
    alias: Option<AttributeSpanWrapper<String>>,
}

impl Field {
    pub fn from_struct_field(field: &SynField, index: usize) -> Result<Self> {
        let SynField {
            ident, attrs, ty, ..
        } = field;

        let mut alias = None;

        for attr in attr::parse_attributes(attrs)? {
            let attribute_span = attr.attribute_span;

            match attr.item {
                FieldAttr::Alias(_, value) => {
                    alias = Some(AttributeSpanWrapper {
                        item: value.value(),
                        attribute_span,
                    })
                }
            }
        }

        Ok(Self {
            _field: field.clone(),
            _index: index,
            ident: ident.clone(),
            ty: ty.clone(),
            alias,
        })
    }

    pub fn is_type(&self, type_name: &str) -> bool {
        if let Type::Path(type_path) = &self.ty {
            if let Some(segment) = type_path.path.segments.last() {
                return segment.ident == type_name;
            }
        }

        return false;
    }

    pub fn inner_type(&self) -> Option<&syn::Type> {
        if let Type::Path(type_path) = &self.ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Vec" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                            return Some(inner_type);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn alias_name(&self) -> String {
        if let Some(attr) = &self.alias {
            attr.item.clone()
        } else {
            "".into()
        }
    }
}

use syn::{Field as SynField, GenericArgument, Ident, PathArguments, Result, Type};

use crate::attr::{self, AttributeSpanWrapper};

pub struct Field {
    pub ty: Type,
    pub ident: Option<Ident>,
    alias: Option<AttributeSpanWrapper<String>>,
}

impl Field {
    pub fn from_struct_field(field: &SynField, _index: usize) -> Result<Self> {
        let SynField {
            ident, attrs, ty, ..
        } = field;

        let mut alias = None;

        for attr in attr::parse_attributes(attrs)? {
            let attribute_span = attr.attribute_span;

            match attr.item {
                attr::FieldAttr::Alias(_ident, value) => {
                    alias = Some(AttributeSpanWrapper {
                        item: value.value(),
                        attribute_span: attribute_span,
                    })
                }
            }
        }

        Ok(Self {
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

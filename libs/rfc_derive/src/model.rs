use crate::field::Field;
use syn::{
    punctuated::Punctuated, token::Comma, DataStruct, DeriveInput, Field as SynField, Fields,
    FieldsNamed, FieldsUnnamed, Result,
};

pub struct Model {
    fields: Vec<Field>,
}

impl Model {
    pub fn from_item(item: &DeriveInput) -> Result<Self> {
        let DeriveInput { data, .. } = &item;

        let fields = match *data {
            syn::Data::Struct(DataStruct {
                fields: Fields::Named(FieldsNamed { ref named, .. }),
                ..
            }) => Some(named),
            syn::Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
                ..
            }) => Some(unnamed),
            _ => None,
        };

        Ok(Self {
            fields: fields_from_item_data(fields)?,
        })
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

fn fields_from_item_data(fields: Option<&Punctuated<SynField, Comma>>) -> Result<Vec<Field>> {
    fields
        .map(|fields| {
            fields
                .iter()
                .enumerate()
                .map(|(i, f)| Field::from_struct_field(f, i))
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or_else(|| Ok(Vec::new()))
}

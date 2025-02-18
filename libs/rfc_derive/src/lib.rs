use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attr;
mod field;
mod model;
mod result;
mod table;
mod util;

#[proc_macro_derive(RfcResult, attributes(sap))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    result::derive(parse_macro_input!(input))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(RfcTable, attributes(sap))]
pub fn derive_table(input: TokenStream) -> TokenStream {
    table::derive(parse_macro_input!(input))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

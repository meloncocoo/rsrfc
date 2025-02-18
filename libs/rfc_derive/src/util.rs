use syn::{
    parse::{Parse, ParseStream, Result},
    token::Eq,
};

pub const ALIAS_NOTE: &str = "alias = \"EV_TYPE\"";

pub fn parse_eq<T: Parse>(input: ParseStream, help: &str) -> Result<T> {
    if input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            format!(
                "unexpected end of input, expected `=`\n\
                 help: The correct format looks like `#[sap({help})]`",
            ),
        ));
    }

    input.parse::<Eq>()?;
    input.parse()
}

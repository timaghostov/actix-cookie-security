use proc_macro2::{Group, Span};
use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, Result, Token,
};

use super::Attibutes;

const UNAUTHORIZED: &str = "unauthorized_function";
const FORBIDDEN: &str = "forbidden_function";

impl Parse for Attibutes {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = extract_session(input)?;

        pass_symbol::<Token![,]>(input)?;

        let roles = extract_roles(input)?;

        let is_finish = pass_symbol::<Token![,]>(input).is_err();
        if is_finish {
            return Ok(Self {
                session,
                roles,
                unauthorized_function: None,
                forbidden_function: None,
            });
        };

        let unauthorized_function = extract_unauthorized_function(input)?;

        let is_finish = pass_symbol::<Token![,]>(input).is_err();
        if is_finish {
            return Ok(Self {
                session,
                roles,
                unauthorized_function: Some(unauthorized_function),
                forbidden_function: None,
            });
        };

        let forbidden_function = extract_forbidden_function(input)?;

        Ok(Self {
            session,
            roles,
            unauthorized_function: Some(unauthorized_function),
            forbidden_function: Some(forbidden_function),
        })
    }
}

fn extract_session(input: ParseStream) -> Result<Ident> {
    input.parse::<Ident>()
}

fn pass_symbol<S: Parse>(input: ParseStream) -> Result<()> {
    input.parse::<S>()?;
    Ok(())
}

fn extract_roles(input: ParseStream) -> Result<proc_macro2::Group> {
    input.parse::<Group>()
}

fn extract_unauthorized_function(input: ParseStream) -> Result<Ident> {
    let attr_name = match input.parse::<Ident>() {
        Ok(attr_name) => attr_name,
        Err(_) => {
            return Err(Error::new(
                Span::call_site(),
                format!("Unknown {UNAUTHORIZED} attribute"),
            ))
        }
    };
    if attr_name.to_string().as_str() != UNAUTHORIZED {
        return Err(Error::new(
            Span::call_site(),
            format!("Unknown {UNAUTHORIZED} attribute"),
        ));
    }

    pass_symbol::<Token![=]>(input)?;

    input.parse::<Ident>()
}

fn extract_forbidden_function(input: ParseStream) -> Result<Ident> {
    let attr_name = match input.parse::<Ident>() {
        Ok(attr_name) => attr_name,
        Err(_) => {
            return Err(Error::new(
                Span::call_site(),
                format!("Unknown {FORBIDDEN} attribute"),
            ))
        }
    };
    if attr_name.to_string().as_str() != FORBIDDEN {
        return Err(Error::new(
            Span::call_site(),
            format!("Unknown {FORBIDDEN} attribute"),
        ));
    }

    pass_symbol::<Token![=]>(input)?;

    input.parse::<Ident>()
}

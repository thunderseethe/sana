use proc_macro_error::{emit_error};

use syn::Attribute;
use syn::Ident;
use syn::LitInt;
use syn::{Token, LitStr};
use syn::parse::ParseStream;
use syn::parse::Parse;
use syn::parenthesized;
use crate::Spanned;

use sana_core::regex::Regex;
use std::convert::TryFrom;

struct RegexExpr(Regex);

#[derive(Debug, Clone, PartialEq)]
pub enum SanaAttr {
    Regex(RegexAttr),
    Token(TokenAttr),
    Error,
}

pub(crate) fn parse_attr(attr: Attribute) -> Option<Spanned<SanaAttr>> {
    let name = attr.path.get_ident()?.to_string();
    let data = match &*name {
        "regex" => SanaAttr::Regex(
            syn::parse2(attr.tokens)
                .map_err(|e| emit_error!(e))
                .ok()?
        ),
        "token" => SanaAttr::Token(
            syn::parse2(attr.tokens)
                .map_err(|e| emit_error!(e))
                .ok()?
        ),
        "error" =>
            SanaAttr::Error,
        _ => return None
    };

    Some(Spanned {
        data,
        span: attr.bracket_token.span
    })
}

fn parse_regex_not(input: ParseStream) -> syn::Result<Regex> {
    if input.peek(Token![!]) {
        drop(input.parse::<Token![!]>());
        let inner = parse_regex_not(input)?;

        Ok(Regex::Not(Box::new(inner)))
    }
    else {
        let regex: LitStr = input.parse()?;
        let span = regex.span();
        let hir = regex_syntax::Parser::new()
                .parse(&regex.value())
                .map_err(|e| syn::Error::new(span, e))?;
        let regex = Regex::try_from(hir)
            .map_err(|e| syn::Error::new(span, e))?;

        Ok(regex)
    }
}

impl Parse for RegexExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() || input.peek(Token![,]) {
            return Err(input.error("Empty regex"))
        }

        let regex = parse_regex_not(input)?;

        let mut union = vec![];
        while input.peek(Token![&]) {
            drop(input.parse::<Token![&]>()?);
            union.push(parse_regex_not(input)?);
        }

        if union.is_empty() {
            Ok(RegexExpr(regex))
        }
        else {
            let regex = Regex::And(Some(regex).into_iter().chain(union).collect());

            Ok(RegexExpr(regex))
        }
    }
}

struct KeyValue {
    key: Ident,
    value: Value
}

enum Value {
    Int(LitInt)
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        drop(input.parse::<Token![=]>()?);
        let value =
            if input.peek(LitInt) {
                Value::Int(input.parse()?)
            }
            else {
                return Err(input.error("Invalid value"))
            };

        Ok(KeyValue { key, value })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegexAttr {
    pub regex: Regex,
    pub priority: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenAttr {
    pub token: Regex,
    pub priority: usize,
}

impl Parse for RegexAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let input = content;

        let regex = input.parse::<RegexExpr>()?.0;

        if input.is_empty() {
            return Ok(RegexAttr {
                regex,
                priority: 0,
            })
        }
        else {
            drop(input.parse::<Token![,]>()?);
        }

        let mut priority = 0;

        let kvs = input.parse_terminated::<_, Token![,]>(KeyValue::parse)?;
        for kv in kvs {
            match &*kv.key.to_string() {
                "priority" => {
                    let value = match kv.value {
                        Value::Int(i) => i,
                    };

                    priority = value.base10_parse()?;
                },
                _ => return Err(syn::Error::new(
                    kv.key.span(),
                    "Invalid parameter name"
                ))
            }
        }

        Ok(RegexAttr { regex, priority })
    }
}

impl Parse for TokenAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let input = content;

        let token = input.parse::<LitStr>()?;
        let token = Regex::literal_str(&token.value());

        if input.is_empty() {
            return Ok(TokenAttr {
                token,
                priority: 0,
            })
        }
        else {
            drop(input.parse::<Token![,]>()?);
        }

        let mut priority = 0;

        let kvs = input.parse_terminated::<_, Token![,]>(KeyValue::parse)?;
        for kv in kvs {
            match &*kv.key.to_string() {
                "priority" => {
                    let value = match kv.value {
                        Value::Int(i) => i,
                    };

                    priority = value.base10_parse()?;
                },
                _ => return Err(syn::Error::new(
                    kv.key.span(),
                    "Invalid parameter name"
                ))
            }
        }

        Ok(TokenAttr { token, priority })
    }
}

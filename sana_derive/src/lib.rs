use proc_macro::TokenStream;
use proc_macro_error::*;
use proc_macro2::Span;
use syn::{Ident, ItemEnum};

use std::ops::Not;

use sana_core::RuleSet;
use sana_core::{Rule, regex::Regex};

use parser::{parse_attr, TokenAttr, RegexAttr, SanaAttr};

mod parser;
mod generator;

#[derive(Debug, Clone)]
struct Spanned<T> {
    data: T,
    span: Span,
}

#[derive(Debug, Clone)]
struct SanaVariant {
    ident: Ident,
    attrs: Vec<Spanned<SanaAttr>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SanaSpec {
    enum_ident: Ident,
    rules: RuleSet<usize>,
    variants: Vec<Ident>,
    terminal: Ident,
}

fn parse_variant(var: syn::Variant) -> Option<SanaVariant> {
    let ident = var.ident;
    let attrs: Vec<_> = var.attrs.into_iter()
        .filter_map(parse_attr)
        .collect();

    if attrs.is_empty() {
        return None
    }
    if var.fields.is_empty().not() {
        emit_error!(var.fields, "Enum variants with fields are not supported");
        return None
    }

    Some(SanaVariant { ident, attrs })
}

fn join_attrs<T>(attrs: &[Spanned<SanaAttr>], action: T) -> Rule<T> {
    let (regex, priority) = match &attrs[0].data {
        SanaAttr::Regex(RegexAttr { regex, priority }) =>
            (regex.clone(), *priority),
        SanaAttr::Token(TokenAttr { token, priority }) =>
            (token.clone(), *priority),
        _ => unreachable!()
    };

    if regex.is_nullable() {
        emit_error!(
            attrs[0].span, "Nullable regular expression";
            note = "The regular expression should not match the empty string, but it does"
        );
    }

    let mut union = vec![];
    for attr in &attrs[1..] {
        let (regex, prio) = match &attrs[0].data {
            SanaAttr::Regex(RegexAttr { regex, priority }) =>
                (regex.clone(), *priority),
            SanaAttr::Token(TokenAttr { token, priority }) =>
                (token.clone(), *priority),
            _ => unreachable!()
        };

        if regex.is_nullable() {
            emit_error!(
                attr.span, "Nullable regular expression";
                note = "The regular expression should not match the empty string, but it does"
            );
        }

        if priority != prio {
            emit_error!(
                attr.span, "Conflicting rule precedences";
                note = "The precedence of the first rule is equal to {}", priority
            );
        }

        union.push(regex);
    }

    let regex =
        if union.is_empty() { regex }
        else { Regex::Or(Some(regex).into_iter().chain(union).collect()) };

    Rule { regex, priority, action }
}

fn build_spec(source: ItemEnum) -> SanaSpec {
    if source.generics.lt_token.is_some() {
        abort!(source.generics, "Generics are not supported")
    }

    let enum_ident = source.ident;
    let mut rules = vec![];
    let mut variants = vec![];
    let mut terminal = None;

    let vars = source.variants.into_iter()
        .filter_map(parse_variant);
    for (i, var) in vars.enumerate() {
        if var.attrs.iter().any(|a| a.data == SanaAttr::Error) {
            if terminal.is_some() {
                emit_error!(var.ident, "More than one #[error] token");

                continue
            }
            else {
                terminal = Some(var.ident);

                continue
            }
        }

        let attrs: Vec<_> = var.attrs.into_iter()
            .filter(|a| a.data != SanaAttr::Error)
            .collect();

        rules.push(join_attrs(&attrs, i));
        variants.push(var.ident)
    }

    if terminal.is_none() {
        abort!(enum_ident, "The enum lacks an #[error] token")
    }

    SanaSpec {
        enum_ident,
        rules: RuleSet { rules },
        variants,
        terminal: terminal.unwrap(),
    }
}

/// Derives lexer for the given enum
///
/// # Attributes
///
/// - `#[error]`: Marks the given variant as the error variant. There must be
/// exactly one error variant for a given enum
/// - `#[regex(re)]`: specify the regular expression corresponding to
/// the given variant
/// - `#[token(tok)]`: specify the string corresponding to the given variant
///
/// Attributes `regex` and `token` can also receive the following parameters:
///
/// - `priority = <integer>` (default is `0`): the priority for the rule
///
/// # Regular expression syntax
///
/// Regular expression, passed to attribute `regex`, has the following syntax:
///
/// ```text
/// regex =
///     regex '&' regex
///     / '!' regex
///     / literal
/// ```
///
/// Here, `literal` is rust string literal containing regular expression using
/// the [regex](https://docs.rs/regex) crate syntax. `&` denotes the intersection
/// of regular expressions while `!` denotes the complement of regular expression.
#[proc_macro_error]
#[proc_macro_derive(Sana, attributes(error, regex, token))]
pub fn sana(input: TokenStream) -> TokenStream {
    let item: ItemEnum = syn::parse(input)
        .expect_or_abort("Sana can be only be derived for enums");

    let spec = build_spec(item);

    abort_if_dirty();

    generator::generate(spec).into()
}

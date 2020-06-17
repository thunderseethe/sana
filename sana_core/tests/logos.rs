use sana_core::{Rule, RuleSet};
use sana_core::regex::Regex;
use sana_core::ir::{Ir, Vm};

use std::convert::TryFrom;

fn compile(rules: &[(&str, &'static str)]) -> Ir<&'static str> {
    let rules: Vec<_> = rules.iter()
        .map(|(regex, act)|  {
            let hir = regex_syntax::Parser::new()
                .parse(regex).unwrap();
            let regex = Regex::try_from(hir).unwrap();

            Rule {
                regex,
                priority: 0,
                action: *act
            }
        })
        .collect();

    let ruleset = RuleSet { rules };
    let dfa = ruleset.construct_dfa();

    Ir::from_automata(dfa)
}

#[test]
fn load() {
    let ir = compile(basic_tokens());
    let mut vm = Vm::new();
    vm.load(&ir);
}

pub fn basic_tokens() -> &'static [(&'static str, &'static str)] {
    &[
        ( r"[ \n\t\f]", "Whitespace" ),
        ( "[a-zA-Z_$][a-zA-Z0-9_$]*", "Identifier" ),
        ( r#""([^"\\]|\\t|\\u|\\n|\\")*""#, "String" ),
        ( "private", "Private" ),
        ( "primitive", "Primitive" ),
        ( "protected", "Protected" ),
        ( "in", "In" ),
        ( "instanceof", "Instanceof" ),
        ( r"\.", "Accessor" ),
        ( r"\.\.\.", "Ellipsis" ),
        ( r"\(", "ParenOpen" ),
        ( r"\)", "ParenClose" ),
        ( r"\{", "BraceOpen" ),
        ( r"\}", "BraceClose" ),
        ( r"\+", "OpAddition" ),
        ( r"\+\+", "OpIncrement" ),
        ( "=", "OpAssign" ),
        ( "==", "OpEquality" ),
        ( "===", "OpStrictEquality" ),
        ( "=>", "FatArrow" ),
    ]
}

pub fn advanced_tokens() -> &'static [(&'static str, &'static str)] {
    &[
        ( r"[ \t\n\f]+", "Whitespace" ),
        ( r#""([^"\\]|\\t|\\u|\\n|\\")*""#, "LiteralString" ),
        ( "0[xX][0-9a-fA-F]+", "LiteralHex" ),
        ( "-?[0-9]+", "LiteralInteger" ),
        ( "[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+", "LiteralFloat" ),
        ( "~", "LiteralNull" ),
        ( "~?", "Sgwt" ),
        ( "~%", "Sgcn" ),
        ( "~[", "Sglc" ),
        ( "~[a-z][a-z]+", "LiteralUrbitAddress" ),
        ( "~[0-9]+-?[\\.0-9a-f]+", "LiteralAbsDate" ),
        ( "(~s[0-9]+(\\.\\.[0-9a-f\\.]+)?)|(~[hm][0-9]+)", "LiteralRelDate" ),
        ( "'", "SingleQuote" ),
        ( "'''", "TripleQuote" ),
        ( "🦀+", "Rustaceans" ),
        ( "[ąęśćżźńół]+", "Polish" ),
        ( r"[\u0400-\u04FF]+", "Cyrillic" ),
        ( r"([#@!\\?][#@!\\?][#@!\\?][#@!\\?])+", "WhatTheHeck" ),
        ( "try|type|typeof", "Keyword" ),
    ]
}

pub fn else_if_tokens() -> &'static [(&'static str, &'static str)] {
    &[
        ( r"[ ]+", "Whitespace" ),
        ( "else", "Else" ),
        ( "else if", "ElseIf" ),
        ( r"[a-z]*", "Other ")
    ]
}

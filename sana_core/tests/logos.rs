use sana_core::{Rule, RuleSet};
use sana_core::regex::Regex;
use sana_core::ir::{Ir, Vm, pprint_ir};

use std::convert::TryFrom;

fn compile(rules: &[(&str, &'static str, usize)]) -> Ir<&'static str> {
    let rules: Vec<_> = rules.iter()
        .map(|(regex, act, prio)|  {
            let hir = regex_syntax::Parser::new()
                .parse(regex).unwrap();
            let regex = Regex::try_from(hir).unwrap();

            Rule {
                regex,
                priority: *prio,
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

#[test]
fn basic_lexer() {
    let ir = compile(basic_tokens());
    // pprint_ir(&ir);
    // panic!();

    let mut vm = Vm::new();
    vm.load(&ir);

    let mut input = "private equal = x == y";

    let gold = &[
        (7, Some("Private")),
        (1, Some("Whitespace")),
        (5, Some("Identifier")),
        (1, Some("Whitespace")),
        (1, Some("OpAssign")),
        (1, Some("Whitespace")),
        (1, Some("Identifier")),
        (1, Some("Whitespace")),
        (2, Some("OpEquality")),
        (1, Some("Whitespace")),
        (1, Some("Identifier")),
        (0, None),
    ];

    for &g in gold {
        let (len, action) = vm.run(input);
        assert_eq!(g, (len, action));

        input = &input[len..];
    }
}

pub fn basic_tokens() -> &'static [(&'static str, &'static str, usize)] {
    &[
        ( r"[ \n\t\f]", "Whitespace", 0 ),
        ( "[a-zA-Z_$][a-zA-Z0-9_$]*", "Identifier", 0 ),
        ( r#""([^"\\]|\\t|\\u|\\n|\\")*""#, "String", 0 ),
        ( "private", "Private", 1 ),
        ( "primitive", "Primitive", 1 ),
        ( "protected", "Protected", 1 ),
        ( "in", "In", 1 ),
        ( "instanceof", "Instanceof", 1 ),
        ( r"\.", "Accessor", 0 ),
        ( r"\.\.\.", "Ellipsis", 0 ),
        ( r"\(", "ParenOpen", 0 ),
        ( r"\)", "ParenClose", 0 ),
        ( r"\{", "BraceOpen", 0 ),
        ( r"\}", "BraceClose", 0 ),
        ( r"\+", "OpAddition", 0 ),
        ( r"\+\+", "OpIncrement", 0 ),
        ( "=", "OpAssign", 0 ),
        ( "==", "OpEquality", 0 ),
        ( "===", "OpStrictEquality", 0 ),
        ( "=>", "FatArrow", 0 ),
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

#![allow(dead_code)]

use sana_core::{regex::{Derivative, Regex}, regex::pprint_regex, Rule, automata::{State, Automata}, RuleSet, ir::{pprint_ir, Ir}};
use regex_syntax;

use std::convert::TryFrom;

mod basic;
mod logos_basic;
mod sql;

fn regex_match(regex: &Regex, input: &str) -> bool {
    let mut regex = regex.clone();
    for ch in input.chars() {
        regex = regex.derivative(ch);
    }

    regex.is_nullable()
}

fn dfa_match<T>(dfa: &Automata<T>, input: &str) -> bool {
    let mut state_ix = 0;
    for ch in input.chars() {
        if let Some(s) = dfa.transite(state_ix, ch) {
            state_ix = s
        }
        else { return false };
    }

    matches!(dfa.get(state_ix), Some(&State::Action(_)))
}

fn print_derivatives(regex: &str, input: &str) {
    let hir = regex_syntax::Parser::new()
        .parse(regex).unwrap();
    let mut regex = Regex::try_from(hir).unwrap();

    pprint_regex(&regex);
    println!();

    for ch in input.chars() {
        regex = regex.derivative(ch);

        pprint_regex(&regex);
        println!();
    }

    println!("Is nullable: {:?}", regex.is_nullable())
}

fn test_basic() {
    let basic = basic::basic();

    for (num, test) in basic.into_iter().enumerate() {
        let hir = regex_syntax::Parser::new()
            .parse(test.0).unwrap();

        let regex =
            if let Ok(r) = Regex::try_from(hir) { r }
            else { continue };

        // pprint_regex(&regex);
        let dfa = Rule { regex, priority: 0, action: 0 }
            .construct_dfa();

        let mut file =
            std::fs::File::create(format!("tmp/graphs/test_{}.dot", num))
            .unwrap();

        dot::render(&dfa, &mut file).unwrap();

        if let Some(span) = test.2[0] {
            let input =
                if let Some(i) = test.1.get(span.0..span.1) { i }
                else { continue };

            print!("{} at {}: ", test.0, input);

            let status =
                if dfa_match(&dfa, input) { "PASS" }
                else { "FAIL "};

            println!("{}", status)
        }
    }
}

fn test_logos_basic() {
    let basic = logos_basic::basic();
    let rules = basic.iter().enumerate()
        .map(|(i, (regex, _))|  {
            let hir = regex_syntax::Parser::new()
                .parse(regex).unwrap();
            let regex = Regex::try_from(hir).unwrap();

            Rule {
                regex,
                priority: 0,
                action: i
            }
        })
        .collect();
    let ruleset = RuleSet { rules };

    let dfa = ruleset.construct_dfa();
    let mut file =
        std::fs::File::create(format!("tmp/logos_basic.dot"))
        .unwrap();

    dot::render(&dfa, &mut file).unwrap();

    let ir = Ir::from_automata(dfa);

    pprint_ir(&ir);
}

fn test_sql() {
    let keywords = sql::keywords();
    let rules = keywords.iter().enumerate()
        .map(|(i, (regex, _))|  {
            let hir = regex_syntax::Parser::new()
                .parse(regex).unwrap();
            let regex = Regex::try_from(hir).unwrap();

            Rule {
                regex,
                priority: 0,
                action: i
            }
        })
        .collect();
    let ruleset = RuleSet { rules };

    let dfa = ruleset.construct_dfa();
    let mut file =
        std::fs::File::create(format!("tmp/sql_keywords.dot"))
        .unwrap();

    dot::render(&dfa, &mut file).unwrap();
}

fn main() {
    // test_basic();
    // test_sql();
    test_logos_basic()
    // print_derivatives("(..)*c", "abc");
}

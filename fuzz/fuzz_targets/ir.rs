#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};

use sana_core::{Rule, RuleSet};
use sana_core::regex::Regex;
use sana_core::ir::Ir;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
#[derive(arbitrary::Arbitrary)]
pub struct Action(u8);

#[derive(Clone, Debug)]
pub struct Priority(u8);

impl Arbitrary for Priority {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Priority> {
        let priority = <u8 as Arbitrary>::arbitrary(u)? & 1;
        Ok(Priority(priority))
    }
}

fuzz_target!(|rules: Vec<(String, Action, Priority)>| {
    let rules: Vec<_> = rules.into_iter()
        .filter_map(|(regex, act, prio)| {
            let hir = regex_syntax::Parser::new()
                .parse(&regex).ok()?;
            let regex = Regex::try_from(hir).ok()?;

            Some(Rule {
                regex,
                priority: prio.0 as usize,
                action: act
            })
        })
        .collect();

    let ruleset = RuleSet { rules };
    let dfa = ruleset.construct_dfa();

    if let Ok(dfa) = dfa {
        let _ = Ir::from_automata(dfa);
    }
});

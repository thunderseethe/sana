use regex::{ClassSet, Regex, Derivative, RegexVector};
use automata::{State, Automata};
use std::collections::{HashMap, VecDeque};

pub mod regex;
pub mod automata;
pub mod ir;
pub mod dot;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    AmbiguityError(usize, usize)
}

pub struct Rule<T> {
    pub regex: Regex,
    pub priority: usize,
    pub action: T,
}

impl<T: Clone> Rule<T> {
    pub fn construct_dfa(&self) -> Automata<T> {
        let state =
            if self.regex.is_nullable() { State::Action(self.action.clone()) }
            else { State::Normal };

        let mut automata = Automata::new(state);
        let mut queue = VecDeque::<Regex>::new();
        let mut stored = HashMap::<Regex, usize>::new();

        queue.push_back(self.regex.clone());
        stored.insert(self.regex.clone(), 0);

        while let Some(r) = queue.pop_front() {
            let from = *stored.get(&r).unwrap();
            let set = ClassSet::from_regex(&r);

            for class in set.classes() {
                let dr = r.derivative(class.pick());
                let to =
                    if let Some(&i) = stored.get(&dr) { i }
                    else {
                        let i = stored.len();
                        let state =
                            if dr.is_nullable() { State::Action(self.action.clone()) }
                            else { State::Normal };

                        queue.push_back(dr.clone());
                        stored.insert(dr, i);

                        automata.insert_state(state);

                        i
                    };

                for range in class.ranges() {
                    automata.insert_edge(from, to, range)
                }
            }
        }

        automata
    }
}

pub struct RuleSet<T> {
    pub rules: Vec<Rule<T>>
}

impl<T: Clone> RuleSet<T> {
    fn top_rule<I>(&self, mut rule_indices: I) -> Result<Option<&Rule<T>>, Error>
    where I: Iterator<Item=usize> {
        let ix =
            if let Some(ix) = rule_indices.next() { ix }
            else { return Ok(None) };

        let (mut top_ix, mut top_prio) = (ix, self.rules[ix].priority);


        for i in rule_indices {
            use std::cmp::Ordering::*;

            let prio = self.rules[i].priority;
            match prio.cmp(&top_prio) {
                Less => (),
                Equal => return Err(Error::AmbiguityError(top_ix, i)),
                Greater => { top_ix = i; top_prio = prio }
            }
        }

        Ok(Some(&self.rules[top_ix]))
    }

    pub fn construct_dfa(&self) -> Result<Automata<T>, Error> {
        let vector = RegexVector {
            exprs: self.rules.iter().map(|r| r.regex.clone()).collect()
        };
        let state =
            match self.top_rule(vector.nullables()) {
                Ok(Some(rule)) => State::Action(rule.action.clone()),
                Err(_) => panic!("State ambiguity"),
                _ => State::Normal,
            };

        let mut automata = Automata::new(state);
        let mut queue = VecDeque::new();
        let mut stored = HashMap::<_, usize>::new();

        queue.push_back(vector.clone());
        stored.insert(vector, 0);

        while let Some(vec) = queue.pop_front() {
            let from = *stored.get(&vec).unwrap();
            let set = ClassSet::from_vector(&vec);

            for class in set.classes() {
                let dvec = vec.derivative(class.pick());
                let to =
                    if let Some(&i) = stored.get(&dvec) { i }
                    else {
                        let i = stored.len();
                        let state =
                            match self.top_rule(dvec.nullables()) {
                                Ok(Some(rule)) => State::Action(rule.action.clone()),
                                _ => State::Normal,
                            };

                        queue.push_back(dvec.clone());
                        stored.insert(dvec, i);

                        automata.insert_state(state);

                        i
                    };

                for range in class.ranges() {
                    automata.insert_edge(from, to, range)
                }
            }

        }

        Ok(automata)
    }
}

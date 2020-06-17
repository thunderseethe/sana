use std::collections::BTreeMap;
use std::ops::{RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State<T> {
    Normal,
    Action(T),
}

/// Inclusive char range
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharRange {
    pub start: char,
    pub end: char,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeKind {
    Fork,
    Link,
    Leaf,
    Sink,
    Terminal,
}

impl CharRange {
    pub const MIN: Self = CharRange { start: '\0', end: '\0' };
    pub const MAX: Self = CharRange { start: std::char::MAX, end: std::char::MAX };

    pub fn new(start: char, end: char) -> Self {
        assert!(start <= end);

        CharRange { start, end }
    }

    fn contains(&self, ch: char) -> bool {
        ch >= self.start && ch <= self.end
    }
}

fn state_range(state: usize) -> RangeInclusive<(usize, CharRange)> {
    let min = CharRange::MIN;
    let max = CharRange::MAX;

    (state, min)..=(state, max)
}

pub struct Automata<T> {
    pub states: Vec<State<T>>,
    pub edges: BTreeMap<(usize, CharRange), usize>,
}

impl<T> Automata<T> {
    pub(crate) fn new(inital: State<T>) -> Self {
        Automata {
            states: vec![inital],
            edges: BTreeMap::new(),
        }
    }

    pub fn get(&self, ix: usize) -> Option<&State<T>> {
        self.states.get(ix)
    }

    pub fn insert_state(&mut self, state: State<T>) {
        self.states.push(state)
    }

    pub fn insert_edge(&mut self, from: usize, to: usize, range: CharRange) {
        self.edges.insert((from, range), to);
    }

    pub fn transitions_from(&self, state: usize) ->
        impl Iterator<Item=(&CharRange, usize)>
    {
        self.edges.range(state_range(state))
            .map(|(k, &v)| (&k.1, v))
    }

    pub fn transite(&self, state: usize, ch: char) -> Option<usize> {
        self.transitions_from(state)
            .find(|t| t.0.contains(ch))
            .map(|(_, state)| state)
    }

    pub fn find_terminal_node(&self) -> usize {
        for i in 0..self.states.len() {
            let ends: Vec<_> = self.transitions_from(i)
                .filter(|(&ch, _)| ch.start == '\0' && ch.end == std::char::MAX)
                .map(|(_, end)| end)
                .collect();

            if ends.len() == 1 { return ends[0] }
        }

        panic!("Automata without a terminal state")
    }

    pub fn node_kinds(&self) -> Vec<NodeKind> {
        let terminal = self.find_terminal_node();
        let mut coedges = BTreeMap::new();
        for (&(start, range), &end) in self.edges.iter() {
            coedges.insert((end, range), start);
        }

        let mut kinds = vec![NodeKind::Fork; self.states.len()];
        for i in 0..self.states.len() {
            if i == terminal {
                kinds[i] = NodeKind::Terminal;
                continue
            }

            let far_edges = self.transitions_from(i)
                .filter(|&(_, end)| end != i && end != terminal);
            let far_coedges = coedges.range(state_range(i))
                .filter(|(&(end, _), &start)| start != i && end != terminal);

            if far_coedges.count() > 1 {
                kinds[i] = NodeKind::Sink
            }
            else {
                match far_edges.count() {
                    0 =>
                        kinds[i] = NodeKind::Leaf,
                    1 =>
                        kinds[i] = NodeKind::Link,
                    _ => (),
                }
            }
        }

        kinds
    }
}

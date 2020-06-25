use regex_syntax::hir;

use std::{
    hash::{Hash, Hasher},
    convert::TryFrom,
    collections::hash_map::DefaultHasher,
    collections::HashSet,
};

use std::ops::Not;
use crate::automata::CharRange;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class(hir::ClassUnicode);

impl Class {
    fn full() -> Self {
        let range = hir::ClassUnicodeRange::new('\0', '\u{10ffff}');
        let class = Class(hir::ClassUnicode::new(vec![range]));

        assert_ne!(class.0.ranges().len(), 0);

        class
    }

    fn from_literal(literal: char) -> Self {
        let range = hir::ClassUnicodeRange::new(literal, literal);
        let class = Class(hir::ClassUnicode::new(vec![range]));

        assert_ne!(class.0.ranges().len(), 0);

        class
    }

    fn is_empty(&self) -> bool {
        self.0.ranges().is_empty()
    }

    fn contains(&self, ch: char) -> bool {
        for r in self.0.ranges() {
            if (r.start()..=r.end()).contains(&ch) {
                return true
            }
        }

        return false
    }

    pub fn pick(&self) -> char {
        self.0.ranges()[0].start()
    }

    pub fn ranges<'a>(&'a self) -> impl Iterator<Item=CharRange> + 'a {
        self.0.ranges().iter()
            .map(|r| CharRange::new(r.start(), r.end()))
    }
}

impl std::hash::Hash for Class {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for r in self.0.ranges() {
            (r.start(), r.end()).hash(hasher)
        }
    }
}

pub trait Derivative {
    fn derivative(&self, ch: char) -> Self;
    fn class_set(&self) -> ClassSet;
}

/// Regular expression with logical operations
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Regex {
    Nothing,
    Empty,
    Literal(char),
    Class(Class),
    Concat(Vec<Regex>),
    Loop(Box<Regex>),
    Or(Vec<Regex>),
    And(Vec<Regex>),
    Not(Box<Regex>),
    Anything,
}

pub fn pprint_regex(regex: &Regex) {
    print_regex_indent(regex, 0)
}

fn print_regex_indent(regex: &Regex, indent: usize) {
    let ws: String = std::iter::repeat(' ')
        .take(indent)
        .collect();

    match regex {
        Regex::Nothing => println!("{}Nothing", ws),
        Regex::Empty => println!("{}Empty", ws),
        Regex::Literal(l) => println!("{}{}", ws, l),
        Regex::Class(class) => println!("{}{:?}", ws, class),
        Regex::Concat(list) => {
            println!("{}Concat", ws);

            for r in list {
                print_regex_indent(r, indent + 2)
            }
        },
        Regex::Loop(r) => {
            println!("{}Loop", ws);

            print_regex_indent(r, indent + 2)
        },
        Regex::Or(list) => {
            println!("{}Or", ws);

            for r in list {
                print_regex_indent(r, indent + 2)
            }
        },
        Regex::And(list) => {
            println!("{}And", ws);

            for r in list {
                print_regex_indent(r, indent + 2)
            }
        },
        Regex::Not(r) => {
            println!("{}Not", ws);

            print_regex_indent(r, indent + 2)
        },
        Regex::Anything => println!("{}Anything", ws),
    }
}

fn flatten_concat(list: &mut Vec<Regex>) {
    if list.iter().all(|e| matches!(*e, Regex::Concat(_)).not()) {
        return
    }

    let mut new = vec![];

    for e in list.drain(..) {
        if let Regex::Concat(list) = e {
            new.extend(list)
        }
        else {
            new.push(e)
        }
    }

    *list = new
}

fn flatten_or(list: &mut Vec<Regex>) {
    if list.iter().all(|e| matches!(*e, Regex::Or(_)).not()) {
        return
    }

    let mut new = vec![];

    for e in list.drain(..) {
        if let Regex::Or(list) = e {
            new.extend(list)
        }
        else {
            new.push(e)
        }
    }

    *list = new
}

fn flatten_and(list: &mut Vec<Regex>) {
    if list.iter().all(|e| matches!(*e, Regex::And(_)).not()) {
        return
    }

    let mut new = vec![];

    for e in list.drain(..) {
        if let Regex::And(list) = e {
            new.extend(list)
        }
        else {
            new.push(e)
        }
    }

    *list = new
}

impl Regex {
    pub fn literal_str(string: &str) -> Regex {
        if string.is_empty() { return Regex::Empty }

        Regex::Concat(string.chars().map(|ch| Regex::Literal(ch)).collect())
    }

    pub fn normalize(&mut self) {
        match self {
            Regex::Concat(list) => {
                for e in list.as_mut_slice() {
                    e.normalize()
                }

                flatten_concat(list);

                if list.iter().any(|e| *e == Regex::Nothing) {
                    return *self = Regex::Nothing
                }

                list.retain(|e| *e != Regex::Empty);

                if list.len() == 1 {
                    return *self = list[0].clone()
                }
                if list.is_empty() {
                    return *self = Regex::Empty
                }
            },
            Regex::Loop(e) => {
                e.normalize();

                match e.as_mut() {
                    Regex::Loop(inner) =>
                        *e = inner.clone(),
                    Regex::Nothing =>
                        *self = Regex::Empty,
                    Regex::Empty =>
                        *self = Regex::Empty,
                    _ => (),
                }
            },
            Regex::Or(list) => {
                for e in list.as_mut_slice() {
                    e.normalize()
                }

                flatten_or(list);

                if list.iter().any(|e| *e == Regex::Anything) {
                    return *self = Regex::Anything
                }

                list.retain(|e| *e != Regex::Nothing);

                if list.len() == 1 {
                    return *self = list[0].clone()
                }
                if list.is_empty() {
                    return *self = Regex::Nothing
                }

                list.sort_by(|l, r| hash(l).cmp(&hash(r)));
                list.dedup()
            },
            Regex::And(list) => {
                for e in list.as_mut_slice() {
                    e.normalize()
                }

                flatten_and(list);

                if list.iter().any(|e| *e == Regex::Nothing) {
                    return *self = Regex::Nothing
                }

                list.retain(|e| *e != Regex::Anything);

                if list.len() == 1 {
                    return *self = list[0].clone()
                }
                if list.is_empty() {
                    return *self = Regex::Anything
                }

                list.sort_by(|l, r| hash(l).cmp(&hash(r)));
                list.dedup()
            },
            Regex::Not(e) => {
                e.normalize();

                match e.as_mut() {
                    Regex::Not(inner) =>
                        *self = *inner.clone(),
                    Regex::Nothing =>
                        *self = Regex::Anything,
                    Regex::Anything =>
                        *self = Regex::Nothing,
                    _ => ()
                }
            },
            _ => ()
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Regex::Nothing => false,
            Regex::Empty => true,
            Regex::Literal(_) => false,
            Regex::Class(c) => c.is_empty(),
            Regex::Concat(list) =>
                list.iter()
                    .fold(true, |res, e| res && e.is_nullable()),
            Regex::Loop(_) => true,
            Regex::Or(list) =>
                list.iter()
                    .fold(false, |res, e| res || e.is_nullable()),
            Regex::And(list) =>
                list.iter()
                    .fold(true, |res, e| res && e.is_nullable()),
            Regex::Not(e) => e.is_nullable().not(),
            Regex::Anything => true,
        }
    }
}

impl Derivative for Regex {
    fn derivative(&self, ch: char) -> Regex {
        let mut res = match self {
            Regex::Nothing =>
                Regex::Nothing,
            Regex::Empty =>
                Regex::Nothing,
            Regex::Literal(c) =>
                if *c == ch { Regex::Empty }
                else { Regex::Nothing },
            Regex::Class(c) =>
                if c.contains(ch) { Regex::Empty }
                else { Regex::Nothing },
            Regex::Concat(list) => {
                let mut sum = vec![];

                for i in 0..list.len() {
                    let mut conc = vec![
                        list[i].derivative(ch)
                    ];
                    conc.extend(list[i..].iter().skip(1).cloned());

                    sum.push(Regex::Concat(conc));

                    if list[i].is_nullable().not() || i == list.len() {
                        break
                    }
                }

                Regex::Or(sum)
            },
            Regex::Loop(e) => Regex::Concat(vec![
                e.derivative(ch),
                Regex::Loop(e.clone())
            ]),
            Regex::Or(list) => Regex::Or(
                list.iter()
                    .map(|e| e.derivative(ch))
                    .collect()
            ),
            Regex::And(list) => Regex::And(
                list.iter()
                    .map(|e| e.derivative(ch))
                    .collect()
            ),
            Regex::Not(e) =>
                Regex::Not(Box::new(e.derivative(ch))),
            Regex::Anything => Regex::Anything,
        };

        res.normalize();

        res
    }

    fn class_set(&self) -> ClassSet {
        ClassSet::from_regex(self)
    }
}

impl TryFrom<hir::Hir> for Regex {
    type Error = &'static str;

    fn try_from(hir: hir::Hir) -> Result<Regex, Self::Error> {
        use hir::*;

        match hir.into_kind() {
            HirKind::Empty => {
                Ok(Regex::Empty)
            },
            HirKind::Concat(concat) => {
                let mut out = Vec::with_capacity(concat.len());

                fn extend(mir: Regex, out: &mut Vec<Regex>) {
                    match mir {
                        Regex::Concat(nested) => {
                            for child in nested {
                                extend(child, out);
                            }
                        },
                        mir => out.push(mir),
                    }
                }

                for hir in concat {
                    extend(Regex::try_from(hir)?, &mut out);
                }

                Ok(Regex::Concat(out))
            },
            HirKind::Alternation(alternation) => {
                let alternation = alternation
                    .into_iter()
                    .map(Regex::try_from)
                    .collect::<Result<_, _>>()?;

                Ok(Regex::Or(alternation))
            },
            HirKind::Literal(hir::Literal::Unicode(literal)) => {
                Ok(Regex::Literal(literal))
            },
            HirKind::Literal(_) =>
                Err("Only Unicode literals are supported"),
            HirKind::Class(hir::Class::Unicode(class)) =>
                Ok(Regex::Class(Class(class))),
            HirKind::Class(_) =>
                Err("Only Unicode classes are supported"),
            HirKind::Repetition(repetition) => {
                if repetition.greedy.not() {
                    Err("#[regex]: non-greedy parsing is currently unsupported.")?;
                }

                let kind = repetition.kind;
                let mir = Regex::try_from(*repetition.hir)?;

                match kind {
                    RepetitionKind::ZeroOrOne => {
                        Ok(Regex::Or(vec![mir, Regex::Empty]))
                    },
                    RepetitionKind::ZeroOrMore => {
                        Ok(Regex::Loop(Box::new(mir)))
                    },
                    RepetitionKind::OneOrMore => {
                        Ok(Regex::Concat(vec![
                            mir.clone(),
                            Regex::Loop(Box::new(mir)),
                        ]))
                    },
                    RepetitionKind::Range(..) => {
                        Err("#[regex]: {n,m} repetition range is currently unsupported.")?
                    },
                }
            },
            HirKind::Group(group) => {
                Regex::try_from(*group.hir)
            },
            HirKind::WordBoundary(_) => {
                Err("#[regex]: word boundaries are currently unsupported.")?
            },
            HirKind::Anchor(_) => {
                Err("#[regex]: anchors in #[regex] are currently unsupported.")?
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassSet {
    set: HashSet<Class>,
}

impl ClassSet {
    pub fn new() -> ClassSet {
        let mut set = HashSet::new();
        set.insert(Class::full());

        ClassSet { set }
    }

    pub fn from_class(class: &Class) -> ClassSet {
        let mut complement = Class::full();
        complement.0.difference(&class.0);

        let mut set = HashSet::new();
        set.insert(class.clone());
        set.insert(complement);

        ClassSet { set }
    }

    pub fn join(&self, other: &ClassSet) -> ClassSet {
        if other.set.is_empty() { return self.clone() }

        let mut set = HashSet::new();
        for left in &self.set {
            for right in &other.set {
                let mut intersect = left.clone();
                intersect.0.intersect(&right.0);

                set.insert(intersect);
            }
        }

        ClassSet { set }
    }

    pub fn from_regex(regex: &Regex) -> ClassSet {
        match regex {
            Regex::Nothing
            | Regex::Empty
            | Regex::Anything =>
                ClassSet::new(),
            Regex::Literal(literal) => {
                let class = Class::from_literal(*literal);

                ClassSet::from_class(&class)
            },
            Regex::Class(class) =>
                ClassSet::from_class(class),
            Regex::Concat(list) => {
                let mut set = ClassSet::from_regex(&list[0]);
                for i in 1..list.len() {
                    if list[i - 1].is_nullable().not() { break }

                    set = set.join(&ClassSet::from_regex(&list[i]))
                }

                set
            },
            Regex::Loop(e) =>
                ClassSet::from_regex(e),
            Regex::Or(list) =>
                list[1..].iter().fold(
                    ClassSet::from_regex(&list[0]),
                    |set, r| set.join(&ClassSet::from_regex(r))
                ),
            Regex::And(list) =>
                list[1..].iter().fold(
                    ClassSet::from_regex(&list[0]),
                    |set, r| set.join(&ClassSet::from_regex(r))
                ),
            Regex::Not(e) =>
                ClassSet::from_regex(e),
        }
    }

    pub fn from_vector(vec: &RegexVector) -> ClassSet {
        vec.exprs.iter()
            .fold(
                ClassSet::new(),
                |set, e| set.join(&ClassSet::from_regex(e))
            )
    }

    pub fn classes<'a>(&'a self) -> impl Iterator<Item=&'a Class> + 'a {
        self.set.iter()
            .filter(|c| c.is_empty().not())
    }

    /// Pick a char from each class in the set
    pub fn pick_chars<'a>(&'a self) -> impl Iterator<Item=char> + 'a {
        self.set.iter().map(|s| s.pick())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexVector {
    pub exprs: Vec<Regex>,
}

impl RegexVector {
    pub fn derivative(&self, ch: char) -> Self {
        let exprs = self.exprs.iter()
            .map(|e| e.derivative(ch))
            .collect();

        RegexVector { exprs }
    }

    pub fn nullables<'a>(&'a self) -> impl Iterator<Item=usize> + 'a {
        self.exprs.iter().enumerate()
            .filter(|(_, e)| e.is_nullable())
            .map(|(i, _)| i)
    }
}

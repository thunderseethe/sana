use std::collections::VecDeque;

use crate::automata::{Automata, NodeKind, State};

pub struct Ir<T> {
    blocks: Vec<Block<T>>
}

pub enum Block<T> {
    Block(Vec<Op<T>>),
    Func(Vec<Op<T>>),
}

impl<T> Block<T> {
    fn push(&mut self, op: Op<T>) {
        match self {
            Block::Block(ops)
            | Block::Func(ops) =>
                ops.push(op)
        }
    }

    fn ops(&self) -> &[Op<T>] {
        match self {
            Block::Block(ops)
            | Block::Func(ops) =>
                ops
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op<T> {
    /// Jump if matches
    JumpMatches {
        from: char,
        to: char,
        on_success: usize,
    },
    /// Jump if not matches
    JumpNotMatches {
        from: char,
        to: char,
        on_failure: usize,
    },
    LoopMatches {
        from: char,
        to: char,
    },
    /// Just jump
    Jump(usize),
    /// Set current action
    Set(T),
    Halt,
}

pub fn pprint_ir<T: std::fmt::Debug>(ir: &Ir<T>) {
    for (i, block) in ir.blocks.iter().enumerate() {
        match block {
            Block::Block(ops) => {
                println!("l{}:", i);

                for op in ops { pprint_op(op) }
            },
            Block::Func(ops) => {
                println!("l{}(λ):", i);

                for op in ops { pprint_op(op) }
            },
        };
    }
}

fn pprint_op<T: std::fmt::Debug>(op: &Op<T>) {
    use Op::*;

    print!("    ");
    match op {
        JumpMatches { from, to, on_success } =>
            println!("jm {:?} {:?} l{}", from, to, on_success),
        JumpNotMatches { from, to, on_failure } =>
            println!("jnm {:?} {:?} l{}", from, to, on_failure),
        LoopMatches { from, to } =>
            println!("lm {:?} {:?}", from, to),
        Jump(to) =>
            println!("jump l{}", to),
        Set(act) =>
            println!("set {:?}", act),
        Halt =>
            println!("halt"),
    }
}

impl<T: Clone> Ir<T> {
    pub fn from_automata(automata: Automata<T>) -> Ir<T> {
        let terminal = automata.find_terminal_node();
        let node_kinds = automata.node_kinds();

        let mut state_blocks: Vec<Option<usize>> =
            vec![None; automata.states.len()];
        let mut blocks = vec![
            Block::Func::<T>(vec![])
        ];
        state_blocks[0] = Some(0);

        fn insert_block<T>(
            st: usize,
            state_blocks: &mut [Option<usize>],
            blocks: &mut Vec<Block<T>>,
            node_kinds: &[NodeKind]
        ) -> usize {
            if let Some(i) = state_blocks[st] {
                i
            }
            else {
                let i = blocks.len();

                let block =
                    if node_kinds[st] == NodeKind::Sink {
                        Block::Func(vec![])
                    }
                    else { Block::Block(vec![]) };

                blocks.push(block);
                state_blocks[st] = Some(i);

                i
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back(0usize);
        queue.push_back(terminal);

        let terminal_block = insert_block(
            terminal,
            &mut state_blocks,
            &mut blocks,
            &node_kinds
        );

        while let Some(st) = queue.pop_front() {
            let block_ix = state_blocks[st].unwrap();

            if let Some(State::Action(act)) = automata.get(st) {
                blocks[block_ix].push(Op::Set(act.clone()))
            }

            match node_kinds[st] {
                NodeKind::Sink | NodeKind::Fork => {
                    let (loops, next) = automata.transitions_from(st)
                        .partition::<Vec<_>, _>(|&(_, to)| to == st);

                    for (ch, _) in loops {
                        blocks[block_ix].push(Op::LoopMatches {
                            from: ch.start,
                            to: ch.end,
                        });
                    }

                    for (ch, to) in next {
                        if state_blocks[to].is_none() {
                            queue.push_back(to)
                        }

                        let to_block = insert_block(
                            to,
                            &mut state_blocks,
                            &mut blocks,
                            &node_kinds
                        );

                        blocks[block_ix].push(Op::JumpMatches {
                            from: ch.start,
                            to: ch.end,
                            on_success: to_block,
                        });
                    }
                },
                NodeKind::Link => {
                    let (loops, next) = automata.transitions_from(st)
                        .partition::<Vec<_>, _>(|&(_, to)| to == st);

                    for (ch, _) in loops {
                        blocks[block_ix].push(Op::LoopMatches {
                            from: ch.start,
                            to: ch.end,
                        });
                    }

                    for (ch, to) in next {
                        if to == terminal { continue }

                        if state_blocks[to].is_none() {
                            queue.push_back(to)
                        }

                        let to_block = insert_block(
                            to,
                            &mut state_blocks,
                            &mut blocks,
                            &node_kinds
                        );

                        blocks[block_ix].push(Op::JumpNotMatches {
                            from: ch.start,
                            to: ch.end,
                            on_failure: terminal_block,
                        });
                        blocks[block_ix].push(Op::Jump(to_block));
                    }
                },
                NodeKind::Leaf => {
                    for (ch, to) in automata.transitions_from(st) {
                        if to == terminal { continue }

                        blocks[block_ix].push(Op::LoopMatches {
                            from: ch.start,
                            to: ch.end,
                        });
                    }

                    blocks[block_ix].push(Op::Halt)
                },
                NodeKind::Terminal =>
                    blocks[block_ix].push(Op::Halt),
            }
        }

        Ir { blocks }
    }
}

pub struct Vm<T> {
    inst_ptr: usize,
    jump_ptr: usize,
    code: Vec<Op<T>>,
    symbol_map: Vec<usize>,
}

impl<T: Clone> Vm<T> {
    pub fn new() -> Self {
        Vm {
            inst_ptr: 0,
            jump_ptr: 0,
            code: vec![],
            symbol_map: vec![]
        }
    }

    pub fn load(&mut self, ir: &Ir<T>) {
        self.code.clear();
        self.symbol_map.clear();

        self.symbol_map.reserve(ir.blocks.len());

        let mut code_len = 0;
        for block in &ir.blocks {
            self.code.extend(block.ops().iter().cloned());
            self.symbol_map.push(code_len);
            code_len += block.ops().len();
        }

        for op in &mut self.code {
            match op {
                Op::JumpMatches { on_success: loc, .. }
                | Op::JumpNotMatches { on_failure: loc, ..}
                | Op::Jump(loc) =>
                    *loc = self.symbol_map[*loc],
                _ => (),
            }
        }
    }

    pub fn run(&mut self, input: &str) -> (usize, Option<T>) {
        let mut input = input.chars();

        self.inst_ptr = 0;
        self.jump_ptr = 0;

        let mut action = None;
        let mut span = 0;

        let mut cursor_pos = 0;
        let mut cursor =
            if let Some(ch) = input.next() {
                ch
            }
            else { return (0, None) };
        let mut eof = false;

        loop {
            match &self.code[self.inst_ptr] {
                Op::JumpMatches { from, to, on_success } => {
                    if eof { break }

                    if (*from..=*to).contains(&cursor) {
                        cursor_pos += cursor.len_utf8();
                        if let Some(ch) = input.next() {
                            cursor = ch
                        }
                        else { eof = true };

                        self.inst_ptr = *on_success;
                        self.jump_ptr = *on_success;
                    }
                    else {
                        self.inst_ptr += 1;
                    }
                },
                Op::JumpNotMatches { from, to, on_failure } => {
                    if eof { break }

                    if (*from..=*to).contains(&cursor) {
                        self.inst_ptr += 1;
                    }
                    else {
                        cursor_pos += cursor.len_utf8();
                        if let Some(ch) = input.next() {
                            cursor = ch
                        }
                        else { eof = true };

                        self.inst_ptr = *on_failure;
                        self.jump_ptr = *on_failure;
                    }
                },
                Op::LoopMatches { from, to} => {
                    if eof { break }

                    if (*from..=*to).contains(&cursor) {
                        cursor_pos += cursor.len_utf8();
                        if let Some(ch) = input.next() {
                            cursor = ch
                        }
                        else { eof = true };

                        self.inst_ptr = self.jump_ptr;
                    }
                    else {
                        self.inst_ptr += 1;
                    }
                },
                Op::Jump(loc) => {
                    self.inst_ptr = *loc;
                    self.jump_ptr = *loc;
                },
                Op::Set(act) => {
                    action = Some(act.clone());
                    span = cursor_pos;

                    self.inst_ptr += 1;
                },
                Op::Halt => break,
            }
        }

        (span, action)
    }
}

use syn::Ident;
use heck::ShoutySnakeCase;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote};

use sana_core::ir::{Op, Ir};
use crate::SanaSpec;

pub(crate) fn generate(spec: SanaSpec) -> TokenStream {
    let dfa = match spec.rules.construct_dfa() {
        Ok(dfa) => dfa,
        Err(sana_core::Error::AmbiguityError(_, i)) =>
            abort!(spec.variants[i], "Ambiguous rule"),
    };

    let ir = Ir::from_automata(dfa);

    let enum_ident = spec.enum_ident;
    let enum_const_name = enum_ident.to_string()
        .to_shouty_snake_case();
    let ir_var = format_ident!("_{}_IR", enum_const_name);
    let ir_code = generate_ir(&enum_ident, &ir, &spec.variants);
    let bytecode = analyze_ir(&ir);
    dbg!(&bytecode);
    // let rust_code = compile_ir(&ir, &enum_ident, &spec.variants);
    let rust_code = quote! { sana::ir::VmResult::Eoi };
    let error = spec.terminal;

    quote! {
        #[doc(hidden)]
        const #ir_var: &'static [sana::ir::Op<#enum_ident>] = #ir_code;

        impl sana::Sana for #enum_ident {
            const ERROR: Self = #enum_ident::#error;
            const USES_VM: bool = true;

            fn ir() -> &'static [sana::ir::Op<Self>] { #ir_code }
            fn lex<'input>(cursor: &mut sana::ir::Cursor<'input>) -> sana::ir::VmResult<Self> {
                #rust_code
            }
        }
    }
}

fn generate_ir(enum_ident: &Ident, ir: &Ir<usize>, variants: &[Ident]) -> TokenStream {
    let code = ir.flatten();
    let mut ops = vec![];

    for op in code {
        let op = match op {
            Op::Shift => quote! {
                sana::ir::Op::Shift
            },
            Op::JumpMatches { from, to, on_success } => quote! {
                sana::ir::Op::JumpMatches {
                    from: #from,
                    to: #to,
                    on_success: #on_success,
                }
            },
            Op::JumpNotMatches { from, to, on_failure } => quote! {
                sana::ir::Op::JumpNotMatches {
                    from: #from,
                    to: #to,
                    on_failure: #on_failure,
                }
            },
            Op::LoopMatches { from, to } => quote! {
                sana::ir::Op::LoopMatches {
                    from: #from,
                    to: #to,
                }
            },
            Op::Jump(loc) => quote! {
                sana::ir::Op::Jump(#loc)
            },
            Op::Set(act) => {
                let var = &variants[act];

                quote! { sana::ir::Op::Set(#enum_ident::#var) }
            },
            Op::Halt => quote! {
                sana::ir::Op::Halt
            },
        };

        ops.push(op)
    }

    quote! {
        &[
            #(#ops),*
        ]
    }
}

// A block is a list of of statements with additional information.
#[derive(Debug)]
struct Block {
    id: usize,
    // whether there are jumps from inner blocks to the beginning of this block
    is_loop: bool,
    // whether there are multiple jumps into this block
    // and it should not be inlined
    is_func: bool,
    code: Vec<Stmt>,
}

type BlockId = usize;

#[derive(Debug)]
enum Stmt {
    /// Set current action
    Set(usize),
    /// Shift the cursor to the next character
    Shift,
    /// A set of guards to jump to a guard's suitable block
    Match(Match),
    If {
        range: (char, char),
        block: BlockId,
    },
    /// An unconditional jump to the block
    Jump(BlockId),
    /// Halt and return an action, if any
    Halt,
}

/// A set of guards to jump to a guard's suitable block
#[derive(Debug)]
struct Match {
    arms: Vec<MatchArm>
}

/// Jump if matches
#[derive(Debug, Clone)]
struct MatchArm {
    ranges: Vec<(char, char)>,
    block: BlockId,
}

#[derive(Debug)]
pub struct Bytecode {
    blocks: Vec<Block>,
}

fn analyze_ir(ir: &Ir<usize>) -> Bytecode {
    let mut blocks: Vec<Block> = ir.blocks.iter()
        .enumerate()
        .map(|(id, block)| {
            let (is_func, ops) = match block {
                sana_core::ir::Block::Block(ops) => (false, ops),
                sana_core::ir::Block::Func(ops) => (true, ops),
            };

            let mut is_loop = false;

            let mut code = vec![];

            // we collect match guards into a single match statement if they follow each other
            // once the are broken with another statement we should dump the accumulator
            let mut match_acc: Option<Match> = None;

            for op in ops {
                match op {
                    Op::Set(act) => {
                        if let Some(match_acc) = match_acc.take() {
                            // dump match acc
                            code.push(Stmt::Match(match_acc))
                        }
                        code.push(Stmt::Set(*act));
                    },
                    Op::Shift => {
                        if let Some(match_acc) = match_acc.take() {
                            // dump match acc
                            code.push(Stmt::Match(match_acc))
                        }
                        code.push(Stmt::Shift);
                    },
                    Op::JumpMatches { from, to, on_success } => {
                        let match_arm = MatchArm {
                            ranges: vec![(*from, *to)],
                            block: *on_success
                        };
                        if let Some(match_acc) = match_acc.as_mut() {
                            match_acc.arms.push(match_arm);
                        } else {
                            match_acc = Some(Match {
                                arms: vec![match_arm],
                            });
                        }
                    },
                    Op::LoopMatches { from, to } => {
                        is_loop = true;

                        let match_arm = MatchArm {
                            ranges: vec![(*from, *to)],
                            block: id // self block id
                        };
                        if let Some(match_acc) = match_acc.as_mut() {
                            match_acc.arms.push(match_arm);
                        } else {
                            match_acc = Some(Match {
                                arms: vec![match_arm],
                            });
                        }
                    },
                    Op::JumpNotMatches { from, to, on_failure } => {
                        if let Some(match_acc) = match_acc.take() {
                            // dump match acc
                            code.push(Stmt::Match(match_acc))
                        }
                        code.push(Stmt::If { range: (*from, *to), block: *on_failure });
                    },
                    Op::Jump(id) => {
                        if let Some(match_acc) = match_acc.take() {
                            // dump match acc
                            code.push(Stmt::Match(match_acc))
                        }
                        code.push(Stmt::Jump(*id));
                    },
                    Op::Halt => {
                        if let Some(match_acc) = match_acc.take() {
                            code.push(Stmt::Match(match_acc))
                        }
                        code.push(Stmt::Halt);
                    },
                }
            }
            // if the last statement was a match guard, then we should dump match statement
            if let Some(match_acc) = match_acc.take() {
                code.push(Stmt::Match(match_acc))
            }

            Block {
                id,
                is_loop,
                is_func,
                code,
            }
        })
        .collect();

    // Optimize matches
    // Group arms by block id
    for block in blocks.iter_mut() {
        for op in block.code.iter_mut() {
            match op {
                Stmt::Match(match_stmt) => {
                    if !match_stmt.arms.is_empty() {
                        let mut arms = match_stmt.arms.clone();

                        // sort by block
                        arms.sort_by(|l, r| l.block.cmp(&r.block));

                        // group by block
                        let mut new_arms = vec![];
                        let mut current_block = arms[0].block;
                        let mut ranges = vec![];
                        for arm in arms.iter() {
                            if arm.block != current_block {
                                let match_arm = MatchArm {
                                    ranges: ranges.clone(),
                                    block: current_block,
                                };
                                new_arms.push(match_arm);

                                ranges.clear();
                                current_block = arm.block;
                            }
                            ranges.extend(&arm.ranges);
                        }
                        if !ranges.is_empty() {
                            // dump the last arm
                            let match_arm = MatchArm {
                                ranges: ranges,
                                block: current_block,
                            };
                            new_arms.push(match_arm);
                        }

                        match_stmt.arms = new_arms;
                    }
                }
                _ => {
                    // ignore all other statements
                },
            }
        }

    }

    let bytecode = Bytecode { blocks };


    bytecode
}

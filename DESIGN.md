# A brief description of Sana

An overall architecture of Sana can be described with the following diagram.

![](https://gist.githubusercontent.com/suhr/39ace61727acbcec7003a617a5321c86/raw/22bf8311a2a505bd3c1f49af19909a000e364c45/sana-architecture.svg)

## Extended regular expressions

Sana supports a subset of [regex](https://lib.rs/crates/regex) regular expressions extended with intersection and complement operations.

The semantics of this operation can be describes as follows:

- `r1 & r2` matches iff `r1` matches and `r2` matches
- `!r` matches iff r1 does *not* match

For example, `"[[:punct:]]+" & !".*--.*"` matches a sequence of punctuation characters that does not contain `--`.

## DFA construction

Sana uses DFA algorithm described in [Regular-expression derivatives reexamined](https://www.ccs.neu.edu/home/turon/re-deriv.pdf) paper.

The main reason why the derivative approach was chosen instead of traditional approach based on NFA to DFA construction is the simplicity of ERE implementation. It also construct great DFAs for the majority of use cases.

## IR

An IR is designed to facilitate the code generation and simplify debugging. Currently, it is executed by an interpreter while lexing. In the future, it will be directly compiled into Rust code.

There are the IR opcodes:

- `shift`: shift the cursor by one character
- `jm a b N`: if the cursor character matches `a..=b`, jump to the block `N`
- `jnm a b N`: if the cursor character does *not* match `a..=b`, jump to the block `N`
- `lm a b`: if the cursor character matches `a..=b`, jump to the start of the current block
- `jump N`: jump to block `N`
- `set act`: set the current action of `act`
- `halt`: stop the excution and return the current action, if any

## Debugging Sana

To simplify testing and debugging, Sana explicitly separates the core from the derive crate. Also, it provides some addtional tools:

- `Derivative` and `pprint_regex` for debugging regular expressions derivatives
- `Automata::transite` and `Automata::transitions_from` allow for manual DFA walk
- `pprint_ir` and `Vm` allow to test and debug IR

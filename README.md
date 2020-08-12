# Sana
[![Rust](https://github.com/suhr/sana/workflows/Rust/badge.svg)](https://github.com/suhr/sana/actions?query=branch%3Amaster)
[![crates.io](https://img.shields.io/crates/v/sana.svg)](https://crates.io/crates/sana)
[![docs.rs](https://docs.rs/sana/badge.svg)](https://docs.rs/sana)

Sana is a lexer generator for Rust. It provides a easy way to create a lexical analyzer for a language.

Unlike other lexer generator, Sana supports extended regular expressions, which allow you to express your intent more clearly. For example, you can write `"[[:punct:]]+" & !".*--.*"` to represent a sequence of punctuation characters which does not contain `--`.

At compile time, Sana:

- Constructs a deterministic state automata from the token definitions
- Generates IR from the automata
- Compiles IR into Rust code

For an overview of the Sana architecture, see [DESIGN.md](https://github.com/suhr/sana/blob/master/DESIGN.md).

# Example

```rust
use sana::{Sana, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Sana)]
#[backend(rust)] // optional. can be either rust or vm. default is rust
enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9]+")]
    Integer,

    #[token("let", priority = 1)]
    Let,
    #[token("=")]
    Equals,
    #[regex(";")]
    Semicolon,

    #[regex("[ \t\r\n]")]
    Whitespace,
    #[error]
    Error,
}

fn main() {
    let mut lexer = Token::lexer("let answer = 42;");

    assert_eq!(
        lexer.next(),
        Some(Spanned { value: Token::Let, start: 0, end: 3 })
    );
    assert_eq!(lexer.next(),
        Some(Spanned { value: Token::Whitespace, start: 3, end: 4 })
    );
    assert_eq!(lexer.next(),
        Some(Spanned { value: Token::Ident, start: 4, end: 10 })
    );
    assert_eq!(lexer.next(),
        Some(Spanned { value: Token::Whitespace, start: 10, end: 11 })
    );
    assert_eq!(
        lexer.next(),
        Some(Spanned { value: Token::Equals, start: 11, end: 12 })
    );
    assert_eq!(
        lexer.next(),
        Some(Spanned { value: Token::Whitespace, start: 12, end: 13 })
    );
    assert_eq!(
        lexer.next(),
        Some(Spanned { value: Token::Integer, start: 13, end: 15 })
    );
    assert_eq!(
        lexer.next(),
        Some(Spanned { value: Token::Semicolon, start: 15, end: 16 })
    );

    // No tokens left
    assert_eq!(
        lexer.next(),
        None
    );
}
```

You can find more examples in the [`examples`](https://github.com/suhr/sana/tree/master/examples) directory.

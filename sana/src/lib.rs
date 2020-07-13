//! Sana is a crate that allows you to create lexers easily.
//!
//! # Example
//!
//! ```rust
//! use sana::{Sana, Spanned};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Sana)]
//! enum Token {
//!     #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
//!     Ident,
//!     #[regex("[0-9]+")]
//!     Integer,
//!
//!     #[token("let", priority = 1)]
//!     Let,
//!     #[token("=")]
//!     Equals,
//!     #[regex(";")]
//!     Semicolon,
//!
//!     #[regex("[ \t\r\n]")]
//!     Whitespace,
//!     #[error]
//!     Error,
//! }
//!
//! let mut lexer = Token::lexer("let answer = 42;");
//!
//! assert_eq!(
//!     lexer.next(),
//!     Some(Spanned { value: Token::Let, start: 0, end: 3 })
//! );
//! assert_eq!(lexer.next(),
//!     Some(Spanned { value: Token::Whitespace, start: 3, end: 4 })
//! );
//! assert_eq!(lexer.next(),
//!     Some(Spanned { value: Token::Ident, start: 4, end: 10 })
//! );
//! assert_eq!(lexer.next(),
//!     Some(Spanned { value: Token::Whitespace, start: 10, end: 11 })
//! );
//! assert_eq!(
//!     lexer.next(),
//!     Some(Spanned { value: Token::Equals, start: 11, end: 12 })
//! );
//! assert_eq!(
//!     lexer.next(),
//!     Some(Spanned { value: Token::Whitespace, start: 12, end: 13 })
//! );
//! assert_eq!(
//!     lexer.next(),
//!     Some(Spanned { value: Token::Integer, start: 13, end: 15 })
//! );
//! assert_eq!(
//!     lexer.next(),
//!     Some(Spanned { value: Token::Semicolon, start: 15, end: 16 })
//! );
//!
//! // No tokens left
//! assert_eq!(
//!     lexer.next(),
//!     None
//! );
//! ```
//!
//! # Lexing rules
//!
//! Each matched token conforms to the following rules:
//!
//! - It is the the longest possible match of all tokens in the given enum
//! - If there's more than one than one token corresponding to the longest match,
//! then the rule with the highest priority is selected

pub use sana_derive::Sana;
#[doc(hidden)]
pub use sana_core::ir;

use sana_core::ir::{Op, Vm};

/// Trait implemented for an enum representing all tokens.
///
/// The trait implemented by `#[derive(Sana)]`. You should not implement it yourself.
pub trait Sana: Sized + Clone + Copy {
    const ERROR: Self;

    #[doc(hidden)]
    fn ir() -> &'static [Op<Self>];

    /// Create a new `Lexer` that will produce tokens of this type
    fn lexer(input: &str) -> Lexer<'_, Self> {
        Lexer::new(input)
    }
}

/// The `Lexer` is an `Iterator` of tokens
#[derive(Debug, Clone)]
pub struct Lexer<'input, Token: Sana + 'static> {
    vm: Vm<'static, 'input, Token>,
}

impl<'input, Token: Sana> Lexer<'input, Token> {
    /// Create a new `Lexer` on the given input
    ///
    /// **NOTE:** for better type inference it's prefered
    /// to use `Sana::lexer` instead
    pub fn new(input: &'input str) -> Self {
        let ir = Token::ir();
        let vm = Vm::new(ir, input);

        Lexer { vm }
    }

    /// Morth the lexer into another lexer, which scans a different token
    ///
    /// The cursor position of the new lexer is the same as the cursor
    /// position of the old lexer before the metamorphosis
    pub fn morph<Lexeme: Sana + 'static>(self) -> Lexer<'input, Lexeme> {
        let mut lexer = Lexeme::lexer(self.source());
        lexer.rewind(self.position());

        lexer
    }

    /// Set the cursor at position `pos`
    pub fn rewind(&mut self, pos: usize) {
        self.vm.rewind(pos)
    }

    /// The current position of the cursor
    pub fn position(&self) -> usize {
        self.vm.position()
    }

    /// The source string of the lexer
    pub fn source(&self) -> &'input str {
        self.vm.input
    }
}

/// A value (for example, token) together with its range
///
/// The range includes the start but excludes the end, similar to `start..end` ranges
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spanned<T> {
    pub start: usize,
    pub end: usize,
    pub value: T,
}

impl<'input, Token: Sana> Iterator for Lexer<'input, Token> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        use sana_core::ir::VmResult::*;

        let token = match self.vm.run() {
            Action { start, end, action } =>
                Spanned { start, end, value: action },
            Error { start, end } =>
                Spanned { start, end, value: Token::ERROR },
            Eoi => return None,
        };

        Some(token)
    }
}

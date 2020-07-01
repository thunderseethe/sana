pub use sana_derive::Sana;

use sana_core::ir::{Op, Vm};

/// The type implemented by `#[derive(Sana)]`
pub trait Sana: Sized + Clone {
    const ERROR: Self;

    #[doc(hidden)]
    fn ir() -> &'static [Op<Self>];

    /// Create a new `Lexer` that will produce tokens of this type
    fn lexer<'input>(input: &'input str) -> Lexer<'input, Self> {
        Lexer::new(input)
    }
}

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
}

/// Token together with its range
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spanned<T> {
    pub start: usize,
    pub end: usize,
    pub token: T,
}

impl<'input, Token: Sana> Iterator for Lexer<'input, Token> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        use sana_core::ir::VmResult::*;

        let token = match self.vm.run() {
            Action { start, end, action } =>
                Spanned { start, end, token: action },
            Error { start, end } =>
                Spanned { start, end, token: Token::ERROR },
            Eof => return None,
        };

        Some(token)
    }
}

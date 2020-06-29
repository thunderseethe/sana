pub use sana_derive::Sana;

use sana_core::ir::{Op, Vm};

pub trait Sana: Sized + Clone {
    const ERROR: Self;

    #[doc(hidden)]
    fn ir() -> &'static [Op<Self>];

    fn lexer<'input>(input: &'input str) -> Lexer<'input, Self> {
        Lexer::new(input)
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'input, Token: Sana + 'static> {
    vm: Vm<'static, 'input, Token>,
}

impl<'input, Token: Sana> Lexer<'input, Token> {
    pub fn new(input: &'input str) -> Self {
        let ir = Token::ir();
        let vm = Vm::new(ir, input);

        Lexer { vm }
    }
}

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

use sana_derive::Sana;

#[derive(Clone, Copy, Sana)]
enum Token {
    #[regex("token")]
    Token1,

    #[regex("token")]
    Token2,

    #[error]
    Error,
}

fn main() { }

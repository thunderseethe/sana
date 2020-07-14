use sana_derive::Sana;

#[derive(Clone, Copy, Sana)]
enum Token {
    #[regex("")]
    Nothing1,

    #[regex(".*")]
    Nothing2,

    #[token("")]
    Nothing3,

    #[error]
    Error,
}

fn main() { }

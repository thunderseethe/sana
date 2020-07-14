use sana_derive::Sana;

#[derive(Clone, Copy, Sana)]
enum Token {
    #[token("token")]
    WithFields(i32),

    #[error]
    Error,
}

fn main() { }

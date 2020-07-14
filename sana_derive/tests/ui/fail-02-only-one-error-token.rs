use sana_derive::Sana;

#[derive(Clone, Copy, Sana)]
enum Token {
    #[error]
    Error1,

    #[error]
    Error2,
}

fn main() { }

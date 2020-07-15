use sana_derive::Sana;

#[derive(Clone, Copy, Sana)]
enum MixErrorRegex {
    #[regex("x")]
    #[error]
    Mixed,

    #[error]
    Error,
}

#[derive(Clone, Copy, Sana)]
enum DoubleErrorAttr {
    #[error]
    #[error]
    Error,
}

fn main() { }

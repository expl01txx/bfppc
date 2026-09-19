use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Multiply,

    #[token(">")]
    ShiftRight,

    #[token("<")]
    ShiftLeft,

    #[token(".")]
    Print,

    #[token(",")]
    Input,

    #[token("[")]
    LoopStart,

    #[token("]")]
    LoopEnd,

    #[regex(r"'[^']'", |lex| lex.slice().chars().nth(1))]
    Char(char),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u64>().ok())]
    Number(u64),
}

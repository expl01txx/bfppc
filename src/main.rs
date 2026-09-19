use clap::Parser;
use logos::Logos;

use crate::lexer::Token;

pub mod codegen;
pub mod elf;
pub mod lexer;
pub mod parser;
pub mod optimizer;

#[derive(Parser, Debug)]
#[command(
    name = "bfppc",
    version = "0.1",
    about = "A lightweight brainfuck++ compiler for linux x64"
)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long, default_value_t = false)]
    opt: bool,
}

fn main() {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.input).unwrap();

    let mut lexer = Token::lexer(&source);

    let mut instructions = parser::parse(&mut lexer);

    if args.opt {
        instructions = optimizer::optimize(&instructions);
    }
    
    let code = codegen::generate_code(&instructions).unwrap();

    std::fs::write(args.output, elf::generate_elf(&code, 0x400000)).unwrap();
}

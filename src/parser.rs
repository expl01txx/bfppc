use logos::Lexer;

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Inst {
    Add(u8),
    Sub(u8),
    Print,
    Input,
    ShiftRight(u8),
    ShiftLeft(u8),
    LoopStart,
    LoopEnd,
    Eof,
}

pub fn parse(lexer: &mut Lexer<Token>) -> Vec<Inst> {
    let mut insts = Vec::new();
    let mut depth: u32 = 0;

    loop {
        let inst = parse_instruction(lexer);

        if inst == Inst::Eof {
            break;
        }

        match inst {
            Inst::LoopStart => depth += 1,
            Inst::LoopEnd => {
                if depth == 0 {
                    panic!("Unmatched ']'");
                }
                depth -= 1;
            }
            _ => {}
        }

        insts.push(inst)
    }

    if depth != 0 {
        panic!("Unclosed '[' loop");
    }

    insts
}

fn parse_instruction(lexer: &mut Lexer<Token>) -> Inst {
    if let Some(token) = lexer.next() {
        if let Ok(token) = token {
            let inst = match token {
                Token::Add => Inst::Add(1),
                Token::Sub => Inst::Sub(1),
                Token::Multiply => parse_multiply(lexer),
                Token::ShiftRight => Inst::ShiftRight(1),
                Token::ShiftLeft => Inst::ShiftLeft(1),
                Token::Print => Inst::Print,
                Token::Input => Inst::Input,
                Token::LoopStart => Inst::LoopStart,
                Token::LoopEnd => Inst::LoopEnd,
                _ => {
                    panic!("Invalid token {:?}", token)
                }
            };

            return inst;
        }
    }

    return Inst::Eof;
}

fn parse_multiply(lexer: &mut Lexer<Token>) -> Inst {
    let Some(Ok(command_token)) = lexer.next() else {
        panic!("Invalid multiplier statement");
    };

    let Some(Ok(value_token)) = lexer.next() else {
        panic!("Invalid multiplier value token");
    };

    let value: u8 = match value_token {
        Token::Number(n) => u8::try_from(n).map_err(|_| panic!("Invalid multiplier value")).unwrap(),
        Token::Char(c) => c as u8,
        _ => panic!("Invalid multiplier value"),
    };

    let inst = match command_token {
        Token::Add        => Inst::Add(value),
        Token::Sub        => Inst::Sub(value),
        Token::ShiftLeft  => Inst::ShiftLeft(value),
        Token::ShiftRight => Inst::ShiftRight(value),
        _ => panic!("Invalid multiplier value command"),
    };

    inst
}

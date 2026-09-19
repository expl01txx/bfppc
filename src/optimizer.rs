use crate::parser::Inst;

fn clone_inst(inst: &Inst) -> Inst {
    match inst {
        Inst::Add(n) => Inst::Add(*n),
        Inst::Sub(n) => Inst::Sub(*n),
        Inst::Print => Inst::Print,
        Inst::Input => Inst::Input,
        Inst::ShiftRight(n) => Inst::ShiftRight(*n),
        Inst::ShiftLeft(n) => Inst::ShiftLeft(*n),
        Inst::LoopStart => Inst::LoopStart,
        Inst::LoopEnd => Inst::LoopEnd,
        Inst::Eof => Inst::Eof,
    }
}

pub fn optimize(insts: &[Inst]) -> Vec<Inst> {
    let mut out = Vec::new();
    let mut arith: Option<i64> = None;
    let mut shift: Option<i64> = None;

    fn flush_arith(out: &mut Vec<Inst>, arith: &mut Option<i64>) {
        if let Some(total) = arith.take() {

            let m = total.rem_euclid(256) as u8;
            if m != 0 {
                if m <= 128 {
                    out.push(Inst::Add(m));
                } else {
                    out.push(Inst::Sub(0u8.wrapping_sub(m)));
                }
            }
        }
    }

    fn flush_shift(out: &mut Vec<Inst>, shift: &mut Option<i64>) {
        if let Some(total) = shift.take() {
            if total > 0 {
                let mut n = total;
                while n > 255 {
                    out.push(Inst::ShiftRight(255));
                    n -= 255;
                }
                if n > 0 {
                    out.push(Inst::ShiftRight(n as u8));
                }
            } else if total < 0 {
                let mut n = -total;
                while n > 255 {
                    out.push(Inst::ShiftLeft(255));
                    n -= 255;
                }
                if n > 0 {
                    out.push(Inst::ShiftLeft(n as u8));
                }
            }
        }
    }

    for inst in insts {
        match inst {
            Inst::Add(n) => {
                flush_shift(&mut out, &mut shift);
                *arith.get_or_insert(0) += *n as i64;
            }
            Inst::Sub(n) => {
                flush_shift(&mut out, &mut shift);
                *arith.get_or_insert(0) -= *n as i64;
            }
            Inst::ShiftRight(n) => {
                flush_arith(&mut out, &mut arith);
                *shift.get_or_insert(0) += *n as i64;
            }
            Inst::ShiftLeft(n) => {
                flush_arith(&mut out, &mut arith);
                *shift.get_or_insert(0) -= *n as i64;
            }
            _ => {
                flush_arith(&mut out, &mut arith);
                flush_shift(&mut out, &mut shift);
                out.push(clone_inst(inst));
            }
        }
    }

    flush_arith(&mut out, &mut arith);
    flush_shift(&mut out, &mut shift);
    out
}
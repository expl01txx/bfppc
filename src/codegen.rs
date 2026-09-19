use iced_x86::IcedError;
use iced_x86::code_asm::*;

use crate::parser::Inst;

const TAPE_SIZE: u64 = 30_000;

const SYS_WRITE: u64 = 1;
const SYS_MMAP: u64 = 9;
const SYS_EXIT: u64 = 60;

const PROT_READ: u64 = 1;
const PROT_WRITE: u64 = 2;
const MAP_PRIVATE: u64 = 0x02;
const MAP_ANONYMOUS: u64 = 0x20;

pub fn generate_code(insts: &[Inst]) -> Result<Vec<u8>, IcedError> {
    let mut a = CodeAssembler::new(64)?;

    a.mov(rax, SYS_MMAP)?;
    a.xor(rdi, rdi)?; // addr = NULL
    a.mov(rsi, TAPE_SIZE)?; // length
    a.mov(rdx, PROT_READ | PROT_WRITE)?; // prot
    a.mov(r10, MAP_PRIVATE | MAP_ANONYMOUS)?; // flags
    a.mov(r8, -1i64)?; // fd = -1
    a.xor(r9, r9)?; // offset = 0
    a.syscall()?;
    a.mov(r13, rax)?; // r13 = tape pointer

    let mut loops: Vec<_> = Vec::new();

    for inst in insts {
        match *inst {
            Inst::Add(n) => {
                a.add(byte_ptr(r13), (n as i8) as i32)?;
            }
            Inst::Sub(n) => {
                a.sub(byte_ptr(r13), (n as i8) as i32)?;
            }
            Inst::ShiftRight(n) => {
                a.add(r13, n as i32)?;
            }
            Inst::ShiftLeft(n) => {
                a.sub(r13, n as i32)?;
            }
            Inst::Print => {
                a.mov(rax, SYS_WRITE)?;
                a.mov(rdi, 1u64)?;
                a.mov(rsi, r13)?;
                a.mov(rdx, 1u64)?;
                a.syscall()?;
            }
            Inst::Input => {
                a.xor(rax, rax)?; // SYS_READ = 0
                a.xor(rdi, rdi)?; // stdin
                a.mov(rsi, r13)?;
                a.mov(rdx, 1u64)?;
                a.syscall()?;

                let mut skip = a.create_label();
                a.test(rax, rax)?;
                a.jg(skip)?; // if ret > 0, keep byte
                a.mov(byte_ptr(r13), 0i32)?; // EOF/error → 0
                a.set_label(&mut skip)?;
            }
            Inst::LoopStart => {
                let mut start = a.create_label();
                let end = a.create_label();
                a.set_label(&mut start)?;
                a.cmp(byte_ptr(r13), 0i32)?; // if cell == 0, skip to matching ']'
                a.je(end)?;
                loops.push((start, end));
            }
            Inst::LoopEnd => {
                let (start, mut end) = loops.pop().expect("unmatched ']' in codegen");
                a.jne(start)?; // if cell != 0, jump back to matching '['
                a.set_label(&mut end)?;
            }
            _ => panic!("Invalid instruction"),
        }
    }

    a.mov(rax, SYS_EXIT)?;
    a.xor(rdi, rdi)?;
    a.syscall()?;

    a.assemble(0x400000)
}

use std::fmt::Display;

#[derive(Clone)]
pub enum Instruction {
    LDC(f64),
    LDV(usize),
    STV(usize),
    NEG,
    ADD,
    MUL,
    SUB,
    DIV,
    CEQ,
    CGT,
    CLT,
    DUP,
    POP,
    NOP,
    RNG,
    BR(usize),
    BRTRUE(usize),
    BRFALSE(usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = match self {
            Instruction::LDC(value) => {
                format!("ldc:{}", value.to_string())
            }
            Instruction::LDV(value) => {
                format!("ldv:{}", value.to_string())
            }
            Instruction::STV(value) => {
                format!("stv:{}", value.to_string())
            }
            Instruction::NEG => String::from("neg"),
            Instruction::ADD => String::from("add"),
            Instruction::MUL => String::from("mul"),
            Instruction::SUB => String::from("sub"),
            Instruction::DIV => String::from("div"),
            Instruction::CEQ => String::from("ceq"),
            Instruction::CGT => String::from("cgt"),
            Instruction::CLT => String::from("clt"),
            Instruction::DUP => String::from("dup"),
            Instruction::POP => String::from("pop"),
            Instruction::NOP => String::from("nop"),
            Instruction::RNG => String::from("rng"),
            Instruction::BR(value) => {
                format!("br:{}", value.to_string())
            }
            Instruction::BRTRUE(value) => {
                format!("brtrue:{}", value.to_string())
            }
            Instruction::BRFALSE(value) => {
                format!("brfalse:{}", value.to_string())
            }
        };
        write!(f, "{}", instruction)
    }
}

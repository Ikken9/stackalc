use std::fmt::Display;

#[derive(Clone)]
pub enum Instruction {
    LDC(f64),
    NEG,
    ADD,
    MUL,
    SUB,
    DIV,
    CEQ,
    CGT,
    CLT,
    DUP,
    POP
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = match self {
            Instruction::LDC(value) => {
                format!("ldc:{}", value.to_string())
            }
            Instruction::NEG => {
                String::from("neg")
            }
            Instruction::ADD => {
                String::from("add")
            }
            Instruction::MUL => {
                String::from("mul")
            }
            Instruction::SUB => {
                String::from("sub")
            }
            Instruction::DIV => {
                String::from("div")
            }
            Instruction::CEQ => {
                String::from("ceq")
            }
            Instruction::CGT => {
                String::from("cgt")
            }
            Instruction::CLT => {
                String::from("clt")
            }
            Instruction::DUP => {
                String::from("dup")
            }
            Instruction::POP => {
                String::from("pop")
            }
        };
        write!(f, "{}", instruction)
    }
}
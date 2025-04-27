use crate::stackalc::instructions::Instruction;
use rand::Rng;
use ratatui::prelude::*;
use ratatui::widgets::ListState;

pub mod instructions;
mod parser;

#[derive(Clone)]
pub struct Stackalc {
    pub stack: Vec<f64>,
    pub memory: [Option<f64>; 32],
    pub expr: Vec<Instruction>,
    pub instruction_list_state: ListState,
    pub stack_list_state: ListState,
    pub calc_mode: CalcMode,
    pub input_mode: InputMode,
    pub calculator_query: String,
    pub calculator_query_char_idx: usize,
    pub cursor_position: Option<Position>,
    pub exit: bool,
}

impl Default for Stackalc {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            memory: [None; 32],
            expr: Vec::new(),
            instruction_list_state: ListState::default(),
            stack_list_state: ListState::default(),
            calc_mode: CalcMode::INFIX,
            input_mode: InputMode::Normal,
            calculator_query: String::new(),
            calculator_query_char_idx: 0,
            cursor_position: None,
            exit: false,
        }
    }
}

impl Stackalc {
    pub fn execute_selected(&mut self) {
        let instruction_idx = self.instruction_list_state.selected().unwrap();
        if let Some(instruction) = self.expr.get(instruction_idx).cloned() {
            self.execute_instruction(instruction);
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LDC(n) => self.ldc(n),
            Instruction::LDV(n) => self.ldv(n),
            Instruction::STV(n) => self.stv(n),
            Instruction::NEG => self.neg(),
            Instruction::ADD => self.add(),
            Instruction::MUL => self.mul(),
            Instruction::SUB => self.sub(),
            Instruction::DIV => self.div(),
            Instruction::CEQ => self.ceq(),
            Instruction::CGT => self.cgt(),
            Instruction::CLT => self.clt(),
            Instruction::DUP => self.dup(),
            Instruction::POP => self.pop(),
            Instruction::NOP => self.nop(),
            Instruction::RNG => self.rng(),
            Instruction::BR(n) => self.br(n),
            Instruction::BRTRUE(n) => self.brtrue(n),
            Instruction::BRFALSE(n) => self.brfalse(n),
        }
    }

    pub fn ldc(&mut self, n: f64) {
        self.stack.push(n);
    }

    pub fn ldv(&mut self, n: usize) {
        if n <= self.memory.len() {
            if let Some(value) = self.memory[n] {
                self.stack.push(value);
            }
        }
    }

    pub fn stv(&mut self, n: usize) {
        if n <= self.memory.len() {
            if let Some(value) = self.stack.pop() {
                self.memory[n] = Some(value)
            }
        }
    }

    pub fn neg(&mut self) {
        if let Some(value) = self.stack.pop() {
            self.stack.push(-value);
        }
    }

    pub fn add(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();

            let result = first_value + second_value;
            self.stack.push(result);
        }
    }

    pub fn mul(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();

            let result = first_value * second_value;
            self.stack.push(result);
        }
    }

    pub fn sub(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();

            let result = first_value - second_value;
            self.stack.push(result);
        }
    }

    pub fn div(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();

            let result = first_value / second_value;
            self.stack.push(result);
        }
    }

    pub fn ceq(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();
            let are_equals = first_value == second_value;

            if are_equals {
                self.stack.push(1.0);
            } else {
                self.stack.push(0.0);
            }
        }
    }

    pub fn cgt(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();
            let gt = first_value >= second_value;

            if gt {
                self.stack.push(1.0);
            } else {
                self.stack.push(0.0);
            }
        }
    }

    pub fn clt(&mut self) {
        if self.stack.len() >= 2 {
            let first_value = self.stack.pop().unwrap();
            let second_value = self.stack.pop().unwrap();
            let lt = first_value <= second_value;

            if lt {
                self.stack.push(1.0);
            } else {
                self.stack.push(0.0);
            }
        }
    }

    pub fn dup(&mut self) {
        if let Some(top) = self.stack.last() {
            self.stack.push(top.clone());
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn br(&mut self, n: usize) {
        if n <= self.expr.len() {
            self.instruction_list_state.select(Some(n));
            self.execute_selected();
        }
    }

    pub fn brtrue(&mut self, n: usize) {
        if let Some(value) = self.stack.pop() {
            if n <= self.expr.len() && value != 0.0 {
                self.br(n)
            }
        }
    }

    pub fn brfalse(&mut self, n: usize) {
        if let Some(value) = self.stack.pop() {
            if n <= self.expr.len() && value == 0.0 {
                self.br(n)
            }
        }
    }

    pub fn nop(&mut self) {
        // do nothing
    }

    pub fn rng(&mut self) {
        let mut rng = rand::thread_rng();
        let n = rng.random_range(0.0..1.0);
        self.stack.push(n);
    }

    pub fn clear(&mut self) {
        self.expr.clear();
        self.stack.clear();
        for register in &mut self.memory {
            *register = None;
        }
    }
}

#[derive(Clone)]
pub enum CalcMode {
    INFIX,
    POSTFIX, // Reverse Polish Notation
    RAW,
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
}

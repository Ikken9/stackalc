use crate::stackalc::instructions::Instruction;

mod parser;
pub mod instructions;

#[derive(Clone)]
pub struct Stackalc {
    pub stack: Vec<f64>,
    pub expr: Vec<Instruction>,
}

impl Stackalc {
    pub fn new() -> Self {
        Stackalc {
            stack: Vec::new(),
            expr: Vec::new(),
        }
    }
    
    pub fn ldc(&mut self, n: f64) {
        self.stack.push(n);
    }
    
    pub fn neg(&mut self) {
        let value = self.stack.pop().unwrap();
        self.stack.push(-value);
    }

    pub fn add(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value + second_value;
        self.stack.push(result);
    }

    pub fn mul(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value * second_value;
        self.stack.push(result);
    }

    pub fn sub(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value - second_value;
        self.stack.push(result);
    }

    pub fn div(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value / second_value;
        self.stack.push(result);
    }

    pub fn ceq(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value == second_value;
        
        if result {
            self.stack.push(1.0);
        } else {
            self.stack.push(0.0);
        }
    }

    pub fn cgt(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value >= second_value;

        if result {
            self.stack.push(1.0);
        } else {
            self.stack.push(0.0);
        }
    }

    pub fn clt(&mut self) {
        let first_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let second_value = self.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
        let result = first_value <= second_value;

        if result {
            self.stack.push(1.0);
        } else {
            self.stack.push(0.0);
        }
    }

    pub fn dup(&mut self) {
        let top_item = self.stack.last().unwrap();
        self.stack.push(top_item.clone());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
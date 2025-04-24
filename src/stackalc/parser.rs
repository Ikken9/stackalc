use std::collections::VecDeque;
use std::fmt;
use regex::Regex;
use crate::stackalc::instructions::Instruction;
use crate::stackalc::Stackalc;

impl Stackalc {
    pub fn parse_infix(&mut self, input: &String) {
        let mut instructions = Vec::new();

        if let Ok(rpn) = shunting_yard(input.as_str()) {
            for item in rpn {
                match item {
                    OpOrNum::Op('+') => instructions.push(Instruction::ADD),
                    OpOrNum::Op('-') => instructions.push(Instruction::SUB),
                    OpOrNum::Op('*') => instructions.push(Instruction::MUL),
                    OpOrNum::Op('/') => instructions.push(Instruction::DIV),
                    OpOrNum::Num(n) => instructions.push(Instruction::LDC(n)),
                    _ => {},
                }
            }

            self.expr.extend(instructions);
        }
    }

    pub fn parse_postfix(&mut self, input: &String) {
        let mut instructions = Vec::new();

        for token in input.split_whitespace() {
            match token {
                "+" => instructions.push(Instruction::ADD),
                "-" => instructions.push(Instruction::SUB),
                "*" => instructions.push(Instruction::MUL),
                "/" => instructions.push(Instruction::DIV),
                _ => {
                    if let Ok(n) = token.parse::<f64>() {
                        instructions.push(Instruction::LDC(n));
                    }
                }
            }
        }

        self.expr.extend(instructions);
    }

    pub fn parse_raw(&mut self, input: &String) {
        let mut instructions = Vec::new();
        let ldc_re = Regex::new(r"^ldc:(\d+(\.\d+)?)$").unwrap();
        let br_re = Regex::new(r"^br:(\d+(\.\d+)?)$").unwrap();
        for token in input.split_whitespace() {
            if let Some(captures) = ldc_re.captures(token) {
                let n = &captures[1];
                instructions.push(Instruction::LDC(n.parse::<f64>().unwrap()));
            }
            else if let Some(captures) = br_re.captures(token) {
                let n = &captures[1];
                instructions.push(Instruction::BR(n.parse::<usize>().unwrap()));
            }
            else {
                match token {
                    "add" => { instructions.push(Instruction::ADD) }
                    "sub" => { instructions.push(Instruction::SUB) }
                    "mul" => { instructions.push(Instruction::MUL) }
                    "div" => { instructions.push(Instruction::DIV) }
                    "neg" => { instructions.push(Instruction::NEG) }
                    "ceq" => { instructions.push(Instruction::CEQ) }
                    "cgt" => { instructions.push(Instruction::CGT) }
                    "clt" => { instructions.push(Instruction::CLT) }
                    "dup" => { instructions.push(Instruction::DUP) }
                    "pop" => { instructions.push(Instruction::POP) }
                    "rng" => { instructions.push(Instruction::RNG) }
                    _ => {}
                }
            }
        }

        self.expr.extend(instructions);
    }
}

fn shunting_yard(expression: &str) -> Result<Vec<OpOrNum>, ParseError> {
    if expression.trim().is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut output = Vec::new();
    let mut operators = VecDeque::new();
    let mut token_iter = expression.chars().peekable();

    while let Some(&c) = token_iter.peek() {
        if c.is_whitespace() {
            token_iter.next();
        } else if c.is_numeric() || c == '.' {
            let mut num_str = String::new();
            while let Some(&ch) = token_iter.peek() {
                if ch.is_numeric() || ch == '.' {
                    num_str.push(ch);
                    token_iter.next();
                } else {
                    break;
                }
            }
            let num = num_str
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidNumber(num_str))?;
            output.push(OpOrNum::Num(num));
        } else if is_operator(c) || c == '(' || c == ')' {
            token_iter.next();
            match c {
                '(' => operators.push_back(c),
                ')' => {
                    let mut found_paren = false;
                    while let Some(op) = operators.pop_back() {
                        if op == '(' {
                            found_paren = true;
                            break;
                        } else {
                            output.push(OpOrNum::Op(op));
                        }
                    }
                    if !found_paren {
                        return Err(ParseError::MismatchedParentheses);
                    }
                }
                _ => {
                    while let Some(&top) = operators.back() {
                        if top != '(' && precedence(top) >= precedence(c) {
                            output.push(OpOrNum::Op(operators.pop_back().unwrap()));
                        } else {
                            break;
                        }
                    }
                    operators.push_back(c);
                }
            }
        } else {
            return Err(ParseError::UnexpectedCharacter(c));
        }
    }

    while let Some(op) = operators.pop_back() {
        if op == '(' {
            return Err(ParseError::MismatchedParentheses);
        }
        output.push(OpOrNum::Op(op));
    }

    Ok(output)
}

#[derive(Debug)]
pub enum OpOrNum {
    Op(char),
    Num(f64),
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}

#[derive(Debug)]
pub enum ParseError {
    MismatchedParentheses,
    InvalidNumber(String),
    UnexpectedCharacter(char),
    EmptyExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            ParseError::InvalidNumber(num_str) => write!(f, "Invalid number: '{}'", num_str),
            ParseError::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
        }
    }
}

impl std::error::Error for ParseError {}
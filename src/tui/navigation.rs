use ratatui::crossterm::event::{KeyCode, KeyEvent};
use crate::stackalc::instructions::Instruction;
use crate::tui::app::{App, CalcMode, InputMode};

impl App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => {
                match key.code {
                    KeyCode::Tab => { self.input_mode = InputMode::Insert }
                    KeyCode::Char('q') => { self.exit = true }
                    KeyCode::Char('i') => { self.calc_mode = CalcMode::INFIX }
                    KeyCode::Char('p') => { self.calc_mode = CalcMode::POSTFIX }
                    KeyCode::Char('r') => { self.calc_mode = CalcMode::RAW }
                    KeyCode::Char('c') => { self.clear() }
                    KeyCode::Up => {
                        self.select_previous()
                    }
                    KeyCode::Down => {
                        self.select_next();
                        self.iterate_once_forward()
                    }
                    _ => {}
                }
            }
            InputMode::Insert => {
                match key.code {
                    KeyCode::Esc => { self.input_mode = InputMode::Normal }
                    KeyCode::Enter => { self.calculate() }
                    KeyCode::Char(to_insert) => { self.enter_char(to_insert) }
                    KeyCode::Backspace => { self.delete_char() }
                    KeyCode::Left => { self.move_cursor_left() }
                    KeyCode::Right => { self.move_cursor_right() }
                    _ => {}
                }
            }
        }
    }

    pub fn iterate_once_forward(&mut self) {
        let selected_idx = self.instruction_list_state.selected().unwrap();
        if let Some(instruction) = self.stackalc.expr.get(selected_idx) {
            match instruction {
                Instruction::LDC(n) => {
                    self.stackalc.ldc(*n)
                }
                Instruction::NEG => {
                    self.stackalc.neg()
                }
                Instruction::ADD => {
                    self.stackalc.add()
                }
                Instruction::MUL => {
                    self.stackalc.mul()
                }
                Instruction::SUB => {
                    self.stackalc.sub()
                }
                Instruction::DIV => {
                    self.stackalc.div()
                }
                Instruction::CEQ => {
                    self.stackalc.ceq()
                }
                Instruction::CGT => {
                    self.stackalc.cgt()
                }
                Instruction::CLT => {
                    self.stackalc.clt()
                }
                Instruction::DUP => {
                    self.stackalc.dup()
                }
                Instruction::POP => {
                    self.stackalc.pop()
                }
            }
        }
    }
    
    fn clear(&mut self) {
        self.stackalc.expr.clear();
        self.stackalc.stack.clear();
    }

    fn select_next(&mut self) {
        self.instruction_list_state.select_next();
    }
    fn select_previous(&mut self) {
        self.instruction_list_state.select_previous();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.calculator_query_char_idx.saturating_sub(1);
        self.calculator_query_char_idx = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.calculator_query_char_idx.saturating_add(1);
        self.calculator_query_char_idx = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.calculator_query.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.calculator_query
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.calculator_query_char_idx)
            .unwrap_or(self.calculator_query.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.calculator_query_char_idx != 0;
        if is_not_cursor_leftmost {
            let current_index = self.calculator_query_char_idx;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.calculator_query.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.calculator_query.chars().skip(current_index);

            self.calculator_query = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.calculator_query.chars().count())
    }
}
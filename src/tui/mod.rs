use crate::stackalc::{CalcMode, InputMode, Stackalc};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Cell, HighlightSpacing, List, ListItem, Paragraph, Row, Table,
};
use std::io;

pub mod navigation;

impl Stackalc {
    pub fn load_input(&mut self) {
        let query = self.calculator_query.clone();

        match self.calc_mode {
            CalcMode::INFIX => self.parse_infix(&query),
            CalcMode::POSTFIX => self.parse_postfix(&query),
            CalcMode::RAW => self.parse_raw(&query),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
                if let Some(cursor_position) = self.cursor_position {
                    frame.set_cursor_position(cursor_position);
                }
            })?;

            self.handle_events();
        }

        Ok(())
    }

    fn render_input_bar(&mut self, area: Rect, buf: &mut Buffer) {
        let is_empty = self.calculator_query.is_empty();
        let text = if is_empty && self.input_mode != InputMode::Insert {
            Line::from(vec![
                Span::styled(" Press ", Style::default().fg(Color::Rgb(131, 139, 167))),
                Span::styled(
                    "TAB",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Rgb(131, 139, 167)),
                ),
                Span::styled(" to write", Style::default().fg(Color::Rgb(131, 139, 167))),
            ])
        } else {
            Line::from(Span::styled(
                self.calculator_query.as_str(),
                Style::default().fg(Color::White),
            ))
        };

        let search_bar = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if self.input_mode == InputMode::Insert {
                        Style::default()
                            .fg(Color::Rgb(198, 160, 246))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    })
                    .title("Input"),
            )
            .style(Style::default().bg(Color::Rgb(24, 25, 38)));

        if self.input_mode == InputMode::Insert {
            self.cursor_position = Some(Position::new(
                area.x + self.calculator_query_char_idx as u16 + 1,
                area.y + 1,
            ));
        } else {
            self.cursor_position = None;
        }

        search_bar.render(area, buf);
    }

    fn render_instructions(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Instructions"))
            .borders(Borders::ALL)
            .bg(Color::Rgb(24, 25, 38));

        let items: Vec<ListItem> = self
            .expr
            .iter()
            .enumerate()
            .map(|(i, instruction)| {
                let offset = format!("{:#06X}", i);
                let line = format!("{}  {}", offset, instruction.to_string());
                ListItem::from(line)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Rgb(166, 209, 137)),
            )
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.instruction_list_state);
    }

    fn render_stack(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Stack"))
            .borders(Borders::ALL)
            .bg(Color::Rgb(24, 25, 38));

        let items: Vec<ListItem> = self
            .stack
            .iter()
            .map(|n| ListItem::from(n.to_string()))
            .rev()
            .collect();

        let list = List::new(items).block(block);

        StatefulWidget::render(list, area, buf, &mut self.stack_list_state);
    }

    fn render_registers(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Memory"))
            .borders(Borders::ALL)
            .bg(Color::Rgb(24, 25, 38));

        let rows: Vec<Row> = self
            .memory
            .iter()
            .enumerate()
            .map(|(i, value)| {
                let reg_name = format!("r{}", i);
                let value_str = value
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "<empty>".to_string());
                Row::new(vec![Cell::from(reg_name), Cell::from(value_str)])
            })
            .collect();

        let table = Table::new(rows, [Constraint::Length(12), Constraint::Min(12)])
            .block(block)
            .header(
                Row::new(vec!["Register", "Value"])
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .column_spacing(1)
            .row_highlight_style(Style::default());

        Widget::render(table, area, buf);
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(9), Constraint::Min(0)])
            .split(area);

        let calc_mode = {
            match self.calc_mode {
                CalcMode::INFIX => String::from("  INFIX "),
                CalcMode::POSTFIX => String::from(" POSTFIX "),
                CalcMode::RAW => String::from("   RAW  "),
            }
        };

        let calc_mode_paragraph = Paragraph::new(calc_mode)
            .block(Block::default().add_modifier(Modifier::BOLD))
            .style(
                Style::default()
                    .fg(match self.calc_mode {
                        CalcMode::INFIX => Color::Rgb(245, 169, 127),
                        CalcMode::POSTFIX => Color::Rgb(125, 196, 228),
                        CalcMode::RAW => Color::Rgb(202, 158, 230),
                    })
                    .bg(Color::White)
                    .add_modifier(Modifier::REVERSED),
            );

        let actions = String::from(
            "  [TAB] Input | [R] Raw | [I] Infix | [P] Postfix | [C] Clear | [Down] Execute next | [Q] Quit",
        );

        let actions_paragraph = Paragraph::new(actions)
            .block(Block::default().add_modifier(Modifier::BOLD))
            .style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Rgb(54, 58, 79))
                    .add_modifier(Modifier::REVERSED),
            );

        calc_mode_paragraph.render(layout[0], buf);
        actions_paragraph.render(layout[1], buf);
    }

    fn handle_events(&mut self) {
        if let Ok(event) = event::read() {
            if let Event::Key(event) = event {
                if event.kind == KeyEventKind::Press {
                    self.handle_key_event(event)
                }
            }
        }
    }
}

impl Widget for &mut Stackalc {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [input_bar_area, main_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .areas(area);

        self.render_input_bar(input_bar_area, buf);
        self.render_footer(footer_area, buf);

        let [instructions_area, stack_area, memory_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(43),
                Constraint::Percentage(43),
                Constraint::Percentage(14),
            ])
            .areas(main_area);

        self.render_instructions(instructions_area, buf);
        self.render_stack(stack_area, buf);
        self.render_registers(memory_area, buf);
    }
}

use std::io;
use crate::stackalc::Stackalc;

mod stackalc;
mod tui;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let stackalc = Stackalc::default();
    let result = stackalc.run(terminal);
    
    ratatui::restore();
    
    result
}

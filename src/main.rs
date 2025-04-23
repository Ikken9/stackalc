use std::io;
use crate::tui::app::App;

mod stackalc;
mod tui;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app = App::new();
    let result = app.run(terminal);
    
    ratatui::restore();
    
    result
}

mod terminal;

use std::io;

fn main() {
    let mut terminal = terminal::init_terminal();
    terminal::draw_terminal(&mut terminal);
}

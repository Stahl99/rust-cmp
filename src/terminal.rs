use std::io::{stdin, stdout, Write, Error};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    let mut stdout = stdout()/*.into_raw_mode().unwrap()*/;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend);

    return terminal.unwrap();
}

pub fn draw_terminal(terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> () {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        Block::default()
             .title("Block")
             .borders(Borders::ALL)
             .render(&mut f, chunks[0]);
        Block::default()
             .title("Block 2")
             .borders(Borders::ALL)
             .render(&mut f, chunks[2]);
    });
}
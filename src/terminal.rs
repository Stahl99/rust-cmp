use std::io::{stdin, stdout, Write, Error};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders, List, Text, ListState};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    let mut stdout = stdout()/*.into_raw_mode()*/;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.hide_cursor();
    terminal.clear();

    return terminal;
}

pub fn draw_terminal(terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> () {
    /*terminal.draw(|mut f| {
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
             .title(&format!("Block: {}", number.to_string()).to_owned())
             .borders(Borders::ALL)
             .render(&mut f, chunks[0]);
        Block::default()
             .title("Block 2: ")
             .borders(Borders::ALL)
             .render(&mut f, chunks[2]);
    });*/

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
            .title(&format!("Block 1"))
            .borders(Borders::ALL)
            .render(&mut f, chunks[0]);
        Block::default()
            .title("Block 2: ")
            .borders(Borders::ALL)
            .render(&mut f, chunks[2]);

        let style = Style::default().fg(Color::White).bg(Color::Blue);

        let mut items = ["Item 1", "Item 2", "Item 3"].iter().map(|i| Text::raw(*i));
        let mut items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .style(style)
                /*.highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))*/
                /*.highlight_symbol(">")*/;
            f.render(&mut items, chunks[0]);
        });
}
use crate::util::app::App;

use std::io::{stdout};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders, List, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
};

pub fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    let stdout = stdout()/*.into_raw_mode()*/;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.hide_cursor();
    terminal.clear();
    
    return terminal;
}

pub fn draw_terminal(terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>, app : &mut App) -> () {
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

        let style = Style::default().fg(Color::White).bg(Color::Black);

        //let mut items = ["Item 1", "Item 2", "Item 3"].iter().map(|i| Text::raw(*i));
        
        let mut items = app.item_list.items.iter().map(|i| Text::raw(*(&i.as_str())));

        let mut items2 = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .style(style);

                f.render(&mut items2, chunks[0]);
        });

        // TEST CODE
        //let s : &str = "Test123";
        //app.item_list.items[0] = s.to_string();

        //app.item_list.next();
        // TEST CODE

}

pub fn run_terminal (app : &mut App) {


    if (app.poll_down()) {
        app.item_list.next();
    }

    if (app.poll_up()) {
        app.item_list.previous();
    }

    // remove "> " from all elements
    for i in 0..app.item_list.items.len() {
        if (app.item_list.items[i].chars().nth(0).unwrap() == '>') {
            app.item_list.items[i] = app.item_list.items[i][2..].to_string();
        }
    }

    // Add "> " to selected element
    let mut selected_element_index : usize = app.item_list.state.selected().unwrap();
    let selected_element : String  = app.item_list.items[selected_element_index].to_string();

    let mut concatenated_element : String = "> ".to_owned();
    concatenated_element = concatenated_element + &selected_element;
    app.item_list.items[selected_element_index] = concatenated_element;

}
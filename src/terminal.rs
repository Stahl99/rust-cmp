use crate::util::app::App;

use std::io::{stdout};
use tui::{
    Terminal, Frame,
    widgets::{Widget, Block, Borders, List, Text, Tabs},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Color, Style, Modifier},
    backend::{CrosstermBackend, Backend}
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

        terminal.draw(|mut f| {
            // set basic chunks
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Min(15), // minimal sidebar length
                        Constraint::Length(120), // default main window length
                    ].as_ref()
                )
                .split(f.size());
            
            // draw basic blocks
            draw_sidebar(&mut f, app, chunks[0]);
            draw_main_block(&mut f, app, chunks[1]);
            
            });

}

fn draw_sidebar(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let sidebar = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(5), // Views
                Constraint::Length(200), // Playlists
            ]
            .as_ref()
        )
        .split(area);

        draw_view_block(&mut f, app, chunks[0]);
        draw_playlist_block(&mut f, app, chunks[1]);

}

fn draw_view_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let view_str : &str = "Views";

    let view_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let mut items = app.view_list.items.iter().map(|i| Text::raw(*(&i.as_str())));

    let mut items2 = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        .title(view_str)
        .title_style(Style::default().fg(Color::Rgb(0, 148, 255))));

    f.render(&mut items2, area);
} 

fn draw_playlist_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let playlist_str : &str = "Playlists";

    let playlist_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let mut items = app.playlist_list.items.iter().map(|i| Text::raw(*(&i.as_str())));

    let mut items2 = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        .title(playlist_str)
        .title_style(Style::default().fg(Color::Rgb(0, 148, 255))));

    f.render(&mut items2, area);

} 

fn draw_main_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let main_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3), // playbar 
                Constraint::Length(200), // main area
            ]
            .as_ref()
        )
        .split(area);

        draw_play_block(&mut f, app, chunks[0]);
        draw_selection_block(&mut f, app, chunks[1]);

}

fn draw_play_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let play_block = Block::default()
    .title(&format!("RUST CMP"))
    .title_style(Style::default().fg(Color::Rgb(216, 127, 26)))
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(5), // buttons
                Constraint::Length(100), // track desc
                Constraint::Min(50), // progress
            ]
            .as_ref()
        )
        .split(area);

        draw_button_block(f, app, chunks[0]);
        draw_track_block(f, app, chunks[1]);
        draw_progress_block(f, app, chunks[2]);

}

fn draw_button_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{}

fn draw_track_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{}

fn draw_progress_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{}

fn draw_selection_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let selection_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

}

pub fn run_terminal (app : &mut App) {


    if (app.poll_down()) {
        app.item_list.next();
        app.view_list.next();
    }

    if (app.poll_up()) {
        app.item_list.previous();
        app.view_list.previous();
    }

    // remove "> " from all elements
    for i in 0..app.view_list.items.len() {
        if (app.view_list.items[i].chars().nth(0).unwrap() == '>') {
            app.view_list.items[i] = app.view_list.items[i][2..].to_string();
        }
    }

    // Add "> " to selected element
    let mut selected_element_index : usize = app.view_list.state.selected().unwrap();
    let selected_element : String  = app.view_list.items[selected_element_index].to_string();

    let mut concatenated_element : String = "> ".to_owned();
    concatenated_element = concatenated_element + &selected_element;
    app.view_list.items[selected_element_index] = concatenated_element;

}
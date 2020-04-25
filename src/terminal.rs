use crate::util::app::App;
use crate::util::app::CurrentElement;

use std::io::{stdout};
use tui::{
    Terminal, Frame,
    widgets::{Widget, Block, Borders, List, Text, Tabs},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Color, Style, Modifier},
    backend::{CrosstermBackend, Backend}
};

pub fn init_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {

    // init basic terminal objects
    let stdout = stdout()/*.into_raw_mode()*/;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.hide_cursor(); // hide cursor
    terminal.clear(); // clear terminal
    
    return terminal;
}

// function that initiates calls to all other sub draw functions
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

// draws the sidebar of the UI
fn draw_sidebar(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let sidebar = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3), // Views
                Constraint::Length(200), // Playlists
            ]
            .as_ref()
        )
        .split(area);

        draw_view_block(&mut f, app, chunks[0]);
        draw_playlist_block(&mut f, app, chunks[1]);

}

// draws a list of all selectable views
fn draw_view_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    // string that is printed later as a title
    let rust_cmp_str : &str = "RUST COMMANDLINE MUSIC PLAYER (CMP)";

    let view_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.view_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::Views {
        app.view_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.view_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    // create render object from item list
    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(rust_cmp_str)
        .title_style(Style::default().fg(app.header_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::Views {
        app.view_list.remove_highlighting_element('>'); 
    }

} 

// draws the playlist selection in the sidebar
fn draw_playlist_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    // string that is printed later as a title
    let playlist_str : &str = "Playlists";

    let playlist_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.playlist_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::Playlists {
        app.playlist_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.playlist_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(playlist_str)
        .title_style(Style::default().fg(app.title_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::Playlists {
        app.playlist_list.remove_highlighting_element('>'); 
    }

} 

// draw the playbar and the main table in the center of the screen
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

// draw the playbar in the center top of the screen
fn draw_play_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let play_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // tab block
                Constraint::Percentage(50), // timeline
            ]
            .as_ref()
        )
        .split(area);

    draw_tab_block(f, app, chunks[0]);
    draw_timeline_block(f, app, chunks[1]);    

}

// draw tabs used to select buttons at the top of the screen (play/pause etc.)
fn draw_tab_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let mut tabs = Tabs::default()
        .block(Block::default().borders(Borders::ALL))
        .titles(&app.playbar_state.titles) // set the content of the tabs items
        .select(app.playbar_state.index) // select an initial item
        .highlight_style(Style::default().fg(app.title_color)) // color used to highlight selected items
        .divider(" "); // defines the divider element between the tab elements

    f.render(&mut tabs, area);

}

// draws the timeline in the top right of the screen
fn draw_timeline_block(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{}

// draws the big table in the center of the screen used to select music
fn draw_selection_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect)
{

    let selection_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    let chunks = Layout::default() 
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40), // tracks
                Constraint::Percentage(25), // artists
                Constraint::Percentage(25), // albums
                Constraint::Percentage(10), // length
            ]
            .as_ref()
        )
        .split(area);

    draw_tracks_block(f, app, chunks[0]);
    draw_artist_block(f, app, chunks[1]);
    draw_albums_block(f, app, chunks[2]);
    draw_lengths_block(f, app, chunks[3]);

}

fn draw_tracks_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect) {

    // string that is printed later as a title
    let tracks_str : &str = "Track";

    let tracks_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.tracks_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::MainArea {
        app.tracks_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.tracks_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(tracks_str)
        .title_style(Style::default().fg(app.title_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::MainArea {
        app.tracks_list.remove_highlighting_element('>'); 
    }

}

fn draw_artist_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect) {

    // string that is printed later as a title
    let artists_str : &str = "Artist";

    let artists_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.artist_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::MainArea {
        app.artist_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.artist_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(artists_str)
        .title_style(Style::default().fg(app.title_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::MainArea {
        app.artist_list.remove_highlighting_element('>'); 
    }

}

fn draw_albums_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect) {

    // string that is printed later as a title
    let albums_str : &str = "Album";

    let albums_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.albums_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::MainArea {
        app.albums_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.albums_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(albums_str)
        .title_style(Style::default().fg(app.title_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::MainArea {
        app.albums_list.remove_highlighting_element('>'); 
    }

}

fn draw_lengths_block(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app : &mut App, area : Rect) {
    // string that is printed later as a title
    let lengths_str : &str = "Length";

    let lengths_block = Block::default()
    .borders(Borders::ALL)
    .render(f, area);

    // calculated the content of the on_display object
    // which only contains items currently visible in the UI
    // this also accounts for horizontal and vertical scrolling
    app.lengths_list.calc_on_display(area.width as usize, area.height as usize, app.horizontal_scroll_delay);

    if app.current_element == CurrentElement::MainArea {
        app.lengths_list.add_highlighting_element("> "); // adds the highlighting element to the selected list element
    }

    // get text from all visible list items
    let mut items = app.lengths_list.get_on_display().iter().map(|i| Text::raw(*(&i.as_str())));

    let mut render_list = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        // set the title of the view block
        .title(lengths_str)
        .title_style(Style::default().fg(app.title_color)));

    f.render(&mut render_list, area);

    // removes the highlighting element from the selected list element after rendering
    if app.current_element == CurrentElement::MainArea {
        app.lengths_list.remove_highlighting_element('>'); 
    }

}

// executes the terminal navigation
pub fn terminal_navigation (app : &mut App) {
    
    // get the input values
    let down = app.poll_down();
    let up = app.poll_up();
    let left = app.poll_left();
    let right = app.poll_right();

    // handle program navigation

    // Views
    if app.current_element == CurrentElement::Views {

        if down {

            if app.view_list.is_last_element_selected() {
                app.current_element = CurrentElement::Playlists;
                app.playlist_list.reset_selection();
                return;
            }

            app.view_list.next();
        }

        if up {
            app.view_list.previous();
        }

        if right {
            app.current_element = CurrentElement::Playbar;
            app.playbar_state.index = 0;
            return;
        }

    }

    // Playlists
    if app.current_element == CurrentElement::Playlists {

        if down {
            app.playlist_list.next();
        }

        if up {

            if app.playlist_list.is_first_element_selected() {
                app.current_element = CurrentElement::Views;
                app.view_list.reset_selection();
                return;
            }

            app.playlist_list.previous();
        }

        if right {
            app.current_element = CurrentElement::MainArea;
            reset_main_area_selection(app);
            return;
        }

    }

    // Playbar
    if app.current_element == CurrentElement::Playbar {

        if left {
            if app.playbar_state.index == 0 {
                app.current_element = CurrentElement::Views;
                app.view_list.reset_selection();
                app.playbar_state.index = 5;
                return;
            }

            app.playbar_state.previous();
        }

        if right {
            if app.playbar_state.index == 4 {
                app.current_element = CurrentElement::Timeline;
                app.playbar_state.index = 5;
                return;
            }

            app.playbar_state.next();
        }

        if down {
            app.current_element = CurrentElement::MainArea;
            reset_main_area_selection(app);
            app.playbar_state.index = 5;
            return;
        }

    }

    // Timeline
    if app.current_element == CurrentElement::Timeline {

        if left {
            app.current_element = CurrentElement::Playbar;
            app.playbar_state.index = 0;
            return;
        }

        if down {
            app.current_element = CurrentElement::MainArea;
            reset_main_area_selection(app);
            return;
        }

    }

    // Main Area
    if app.current_element == CurrentElement::MainArea {

        if down {
            app.tracks_list.next();
            app.artist_list.next();
            app.albums_list.next();
            app.lengths_list.next();
        }

        if up {
            if app.tracks_list.is_first_element_selected() {
                app.current_element = CurrentElement::Playbar;
                app.playbar_state.index = 0;
                return;
            }

            app.tracks_list.previous();
            app.artist_list.previous();
            app.albums_list.previous();
            app.lengths_list.previous();
        }

        if left {
            app.current_element = CurrentElement::Playlists;
            app.playlist_list.reset_selection();
            return;
        } 

    }

}

fn reset_main_area_selection (app : &mut App) {

    app.tracks_list.reset_selection();
    app.artist_list.reset_selection();
    app.albums_list.reset_selection();
    app.lengths_list.reset_selection();

}
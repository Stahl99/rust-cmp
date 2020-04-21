mod terminal;
mod util;

use crate::util::{
    app::App,
    app::Event,
    app::Cli
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{backend::CrosstermBackend, Terminal};

use argh::FromArgs;
use std::{
    io,
    thread,
    time::Duration,
    sync::mpsc,
    sync::mpsc::Receiver,
};

fn main() {

    let cli: Cli = argh::from_env();

    enable_raw_mode();

    // initialize terminal objects and hide curosr
    let mut terminal = terminal::init_terminal();
    terminal.hide_cursor();

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(Duration::from_millis(cli.tick_rate)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }

            tx.send(Event::Tick).unwrap();
        }
    });

    // create app with basic values
    let mut app = App::new();
    init_app(&mut app);

    // main program loop
    while !app.should_quit {

        terminal::draw_terminal(&mut terminal, &mut app); // draw the UI
        handle_user_input(&mut app, &mut terminal, &rx); // handle user input
        terminal::run_terminal(&mut app); // execute the terminal logic

    }

    // clear the terminal before exiting the program
    terminal.clear();

}

// inits some app elements
// TODO: move to app.rs
fn init_app (app : &mut App)
{
    app.playbar_state.index = 6; // 6 = empty tab -> nothing is visibly selected

    app.item_list.all_elements.state.select(Some(0));
    app.view_list.all_elements.state.select(Some(0));
    app.playlist_list.all_elements.state.select(Some(0));
}

// handles the user input for the app
fn handle_user_input (app : &mut App, terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>, rx : &Receiver<Event<crossterm::event::KeyEvent>>)
{
    match rx.recv() {
        Ok(Event::Input(event)) => match event.code {
            // check if q has been pressed to exit the program
            KeyCode::Char('q') => {
                disable_raw_mode();
                app.should_quit = true;
                terminal.show_cursor();
            }

            // check the arrow keys and safe the values to 
            // state variables in app
            KeyCode::Left => app.left = true,
            KeyCode::Up => app.up = true,
            KeyCode::Right => app.right = true,
            KeyCode::Down => app.down = true,
            _ => {}
        },

        // removed for now as it is currently unused
        Ok(Event::Tick) => {
            //app.on_tick();
        }

        // exit if the app should quit
        Err(_) => {
            app.should_quit = true;
        }
    }

}
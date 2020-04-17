mod terminal;
mod util;

use crate::util::StatefulList::StatefulList;
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
    sync::mpsc::Sender,
    sync::mpsc::Receiver,
};

fn main() {

    let cli: Cli = argh::from_env();

    enable_raw_mode();

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

    let mut app = App::new();

    while !app.should_quit {

        terminal::draw_terminal(&mut terminal, &mut app);
        handle_user_input(&mut app, &mut terminal, &rx);
        terminal::run_terminal(&mut app);

    }
}

fn handle_user_input (app : &mut App, terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>, rx : &Receiver<Event<crossterm::event::KeyEvent>>)
{
    match rx.recv() {
        Ok(Event::Input(event)) => match event.code {
            KeyCode::Char('q') => {
                disable_raw_mode();
                app.should_quit = true;
                terminal.show_cursor();
            }
            KeyCode::Left => app.left = true,
            KeyCode::Up => app.right = true,
            KeyCode::Right => app.up = true,
            KeyCode::Down => app.down = true,
            _ => {}
        },
        Ok(Event::Tick) => {
            app.on_tick();
        }
        Err(_) => {
            app.should_quit = true;
        }
    }

}
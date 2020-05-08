mod terminal;
mod util;
mod player_interface;
mod player;

use player_interface::PlayerInterface;

use crate::util::{
    app::App,
    app::CmdArgs,
    app::Event,
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
    net::IpAddr,
};

fn main() {

    // parses command line arguments
    let cli: CmdArgs = argh::from_env();

    // checks if IP address is valid
    // this also checks the default address (which should be valid)
    match cli.ip.parse::<IpAddr>() {
        Err(_v) => {println!("Error: IP Address not valid!"); std::process::exit(1)},
        Ok(x) => x,
    };

    // checks if the port is valid
    // this also checks the default port (which should be valid)
    match cli.port.parse::<i16>() {
        Err(_v) => {println!("Error: Port not valid!"); std::process::exit(1)},
        Ok(x) => x,
    };

    // constructs string of ip with port to be used later
    let ip_with_port : String = format!("{}:{}", cli.ip, cli.port);

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

    // create instance of player interface and initialize playlist view
    let mut player_interface = PlayerInterface::new(&ip_with_port);
    player_interface.initialize(&mut app);

    // main program loop
    while !app.should_quit {

        terminal::draw_terminal(&mut terminal, &mut app); // draw the UI
        handle_user_input(&mut app, &mut terminal, &rx, &mut player_interface); // handle user input
        terminal::terminal_navigation(&mut app); // handle the terminal navigation
        player_interface.update_meta_display(&mut app); // update display of title and artist
    }

    // clear the terminal before exiting the program
    terminal.clear();
    player_interface.quit();
}

// handles the user input for the app
fn handle_user_input (mut app : &mut App, terminal : &mut Terminal<CrosstermBackend<std::io::Stdout>>, rx : &Receiver<Event<crossterm::event::KeyEvent>>, player_interface : &mut PlayerInterface)
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

            // check for the Enter key and start the
            // requested action
            KeyCode::Enter => player_interface.user_action(&mut app),

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
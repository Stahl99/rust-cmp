use crate::util::StatefulList::StatefulList;

use argh::FromArgs;

pub struct App<'a> {
    pub item_list: StatefulList<&'a str>,
    pub should_quit: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl<'a> App<'a> {

    pub fn new() -> App<'a> {
        App {
            item_list: StatefulList::with_items(vec![
                "Item0", "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
                "Item9", "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16",
                "Item17", "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
            ]),
            should_quit: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn poll_up (mut self) -> bool {
        if self.up {
            self.up = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_down (mut self) -> bool {
        if self.down {
            self.down = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_left (mut self) -> bool {
        if self.left {
            self.left = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_right (mut self) -> bool {
        if self.right {
            self.right = false;
            return true;
        }
        
        return false;
    }

    // function that gets called every tick
    pub fn on_tick(&mut self) {

    }
}

// +-----------------------------+
// | from tui-rs repo examples:  |
// +-----------------------------+
pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, FromArgs)]
#[argh(description = "...")]
pub struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    pub tick_rate: u64,
}
mod terminal;
mod util;

use crate::util::StatefulList::StatefulList;
use crate::util::app::App;

use std::io;
use std::thread;

fn main() {

    let mut app = App {
        item_list: StatefulList::with_items(vec![
            "Item0", "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
            "Item9", "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16",
            "Item17", "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
        ]),
    };

    let mut terminal = terminal::init_terminal();

    loop {

        terminal::draw_terminal(&mut terminal, &mut app);
        thread::sleep_ms(1000);

    }
}
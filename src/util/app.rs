use crate::util::StatefulList::StatefulList;
use crate::util::StatefulList::TabsState;

use tui::style::Color;

use argh::FromArgs;

pub struct App<'a> {
    // used to hold the contents of the UI lists
    pub item_list: StatefulSelectedList,
    pub view_list: StatefulSelectedList,
    pub playlist_list: StatefulSelectedList,

    pub current_element: CurrentElement, // currently selected UI block
    pub playbar_state: TabsState<'a>, // currently selected playbar element
    pub should_quit: bool, // if set to true the programm exits

    // variables to track user input
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

    // colors used in the UI
    pub header_color: Color,
    pub title_color: Color,

}

impl<'a> App<'a> {

    pub fn new() -> App<'a> {

        App {
            item_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), "Item1".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),
            view_list: StatefulSelectedList::new(vec![
                "Show Tracks".to_string(),
            ]),
            playlist_list: StatefulSelectedList::new(vec![
                "Playlist 1".to_string(), "Playlist 2".to_string(), "Playlist 3".to_string(), "Playlist 4".to_string(), "Playlist 5".to_string(), 
                "Playlist 6".to_string(), "Playlist 7".to_string(), "Playlist 8".to_string(), "Playlist 9".to_string(), "Playlist 10".to_string(), 
                "Playlist 11".to_string(), "Playlist 12".to_string(), "Playlist 13".to_string(), "Playlist 14".to_string(), "Playlist 15".to_string(), 
            ]),

            current_element: CurrentElement::Views,
            // last element is empty so that it can be selected when no element of the tabs should be selected
            playbar_state: TabsState::new(vec!["<<", ">", ">>", "Testtrack", "Testartist", ""]), 
            should_quit: false,

            up: false,
            down: false,
            left: false,
            right: false,

            header_color: Color::Rgb(216, 127, 26),
            title_color: Color::Rgb(0, 148, 255),
        }

    }

    // +---------------------------------------------------------------+
    // | polling functions used to read and reset user input variables |
    // +---------------------------------------------------------------+

    pub fn poll_up (&mut self) -> bool {
        if self.up {
            self.up = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_down (&mut self) -> bool {
        if self.down {
            self.down = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_left (&mut self) -> bool {
        if self.left {
            self.left = false;
            return true;
        }
        
        return false;
    }

    pub fn poll_right (&mut self) -> bool {
        if self.right {
            self.right = false;
            return true;
        }
        
        return false;
    }

}

// this enum is used as a type to tell which part of the
// UI is currently selected
pub enum CurrentElement {
    Views,
    Playlists,
    Playbar,
    Timeline,
    MainArea,
}

// wrapper struct for StatefulList with the addition of
// on_display to contain only the currently visible elements for rendering 
pub struct StatefulSelectedList {
    pub on_display : Vec<String>,
    pub all_elements : StatefulList<String>,
    old_height : usize, // used in the calc_on_display function
}

impl StatefulSelectedList {

    pub fn new (content : Vec<String>) -> StatefulSelectedList {
        StatefulSelectedList {
            on_display: content.to_vec(),
            all_elements: StatefulList::with_items(content),
            old_height: 0
        } 
    }

    pub fn calc_on_display (&mut self, mut list_height : usize) {

        // list_height includes the borders of the block. This has to be corrected
        list_height -= 2; 

        // clear the existing vector and create a new one with the capacity of the 
        // currently visible part of the list
        self.on_display = Vec::with_capacity(list_height);

        // check if the height of the list has changed since last time
        let mut height_changed : bool = false;
        if self.old_height != list_height {
            height_changed = true;
        }
        self.old_height = list_height;

        // reset the offset if the height has changed
        if height_changed {
            self.all_elements.state.offset = 0;
        }

        // check where the selected element lies relative to the currently visible part of the list
        if self.all_elements.state.selected.unwrap() >= list_height + self.all_elements.state.offset {
            self.all_elements.state.offset = self.all_elements.state.selected.unwrap() + 1 - list_height;
        } else if self.all_elements.state.selected.unwrap() < self.all_elements.state.offset {
            self.all_elements.state.offset = self.all_elements.state.selected.unwrap();
        }

        // copy all visible elements of the list to on_display
        for i in 0..self.all_elements.items.len() {
            if i >= self.all_elements.state.offset && i < self.all_elements.state.offset + list_height {
                self.on_display.push(self.all_elements.items[i].to_string());
            }
        }

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
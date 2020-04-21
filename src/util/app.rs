use crate::util::StatefulList::StatefulList;
use crate::util::StatefulList::TabsState;

use tui::style::Color;

use argh::FromArgs;

pub struct App<'a> {
    pub horizontal_scroll_delay: u16,

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
            horizontal_scroll_delay: 1,

            item_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), "Item1".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),
            view_list: StatefulSelectedList::new(vec![
                "Show Tracks".to_string(),
            ]),
            playlist_list: StatefulSelectedList::new(vec![
                "Ein sehr sehr langer String der unnötig lang ist".to_string(), "This documentation describes a number of methods and trait implementations on the char type. For technical reasons, there is additional, separate documentation in the std::char module as well.".to_string(), "Playlist 3".to_string(), "Playlist 4".to_string(), "Playlist 5".to_string(), 
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
    horizontal_tick_counter : u16, // this value is used for horizontal scrolling
    selected_element_index_in_on_display : usize,
}

impl StatefulSelectedList {

    pub fn new (content : Vec<String>) -> StatefulSelectedList {
        StatefulSelectedList {
            on_display: content.to_vec(),
            all_elements: StatefulList::with_items(content),
            old_height: 0,
            horizontal_tick_counter: 0,
            selected_element_index_in_on_display: 0
        } 
    }

    // removes the text highlighting element from all list elements passed to the function
    pub fn remove_highlighting_element(&mut self, highlighting_element : char)
    {

        for i in 0..self.on_display.len() {
            if self.on_display[i].chars().nth(0).unwrap() == highlighting_element {
                self.on_display[i] = self.on_display[i][2..].to_string();
            }
        }

    }

    // adds the highlighting element to the currently selected list item
    pub fn add_highlighting_element(&mut self, highlighting_element : &str)
    {

        //let selected_element_index : usize = list.state.selected().unwrap();
        let selected_element : String  = self.on_display[self.selected_element_index_in_on_display].to_string();

        let mut concatenated_element : String = highlighting_element.to_owned();
        concatenated_element = concatenated_element + &selected_element;
        self.on_display[self.selected_element_index_in_on_display] = concatenated_element;

    }

    pub fn calc_on_display (&mut self, mut list_width : usize, mut list_height : usize, horizontal_scroll_delay : u16) {

        // list_height and width include the borders of the block. This has to be corrected
        list_height -= 2; 
        list_width -= 2;

        // clear the existing vector and create a new one with the capacity of the 
        // currently visible part of the list
        self.on_display = Vec::with_capacity(list_height);

        self.calc_horizontal_scrolling(list_width, horizontal_scroll_delay);
        self.calc_vertical_scrolling(list_height);

    }

    fn calc_horizontal_scrolling (&mut self, list_width : usize, horizontal_scroll_delay : u16) {

        // increment the tick counter
        self.horizontal_tick_counter += 1;

        // check if the counter is still below the horizontal scroll delay
        if self.horizontal_tick_counter < horizontal_scroll_delay {
            return;
        }

        // if the counter has surpassed the threshold:
        // reset counter and scroll the text:
        self.horizontal_tick_counter = 0;

        // iterate over all list items in on_display
        for i in 0..self.all_elements.items.len() {

            // only do something if the text is longer than the space provided by the block
            // and the displayed string is longer that two characters
            if self.all_elements.items[i].chars().count() > list_width && self.all_elements.items[i].chars().count() > 2 {

                // shift all characters in the string to the right 
                // (the leftmost character reappears on the right)
                let mut shifted_string : String = String::with_capacity(self.all_elements.items[i].capacity());

                // getting the index this way is nessecary because the string can be unicode
                let index_of_second_element = self.all_elements.items[i].as_str().char_indices().map(|(i, _)| i).nth(1).unwrap();

                shifted_string.push_str(&self.all_elements.items[i][index_of_second_element..]); // create string without first char
                shifted_string.push(self.all_elements.items[i].chars().nth(0).unwrap()); // append original first element at the back

                self.all_elements.items[i] = shifted_string; // exchange old string for new one

            }
        }

    }

    fn calc_vertical_scrolling (&mut self, list_height : usize) {

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

                // translate the index of the currently selected element to an on_display index
                // save the value in selected_element_index_in_on_display
                if self.all_elements.state.selected().unwrap() == i {
                    self.selected_element_index_in_on_display = self.on_display.len();
                }

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
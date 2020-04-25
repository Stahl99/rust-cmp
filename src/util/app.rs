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

        let mut app = App {
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
                "Playlist 6".to_string(), "Playlist 7".to_string(), "Noch ein extraordinär langer String".to_string(), "Playlist 9".to_string(), "Playlist 10".to_string(), 
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
        };

        app.playbar_state.index = 6; // 6 = empty tab -> nothing is visibly selected

        // Select first element
        app.item_list.all_elements.state.select(Some(0));
        app.view_list.all_elements.state.select(Some(0));
        app.playlist_list.all_elements.state.select(Some(0));

        return app;
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

pub struct ScrollStatus {
    index : usize,
    scroll_status : u16,
}

impl ScrollStatus {

    pub fn new (new_index : usize, new_scroll_status : u16) -> ScrollStatus {
        ScrollStatus {
            index: new_index,
            scroll_status: new_scroll_status,
        }
    }

}

// wrapper struct for StatefulList with the addition of
// on_display to contain only the currently visible elements for rendering 
pub struct StatefulSelectedList {
    on_display : Vec<String>, // all elements currently on display (can be modified (selection, scrolling etc.))
    all_elements_scroll_status : Vec<ScrollStatus>, // index of all elements with corresponding scroll status
    all_elements : StatefulList<String>, // all elements in their raw form
    old_height : usize, // used in the calc_on_display function
    horizontal_tick_counter : u16, // this value is used for horizontal scrolling
    selected_element_index_in_on_display : usize, // translated index of the currently selected element
}

impl StatefulSelectedList {

    pub fn new (content : Vec<String>) -> StatefulSelectedList {
        let mut list = StatefulSelectedList {
            on_display: Vec::new(),
            all_elements_scroll_status: Vec::with_capacity(content.capacity()),
            all_elements: StatefulList::with_items(content),
            old_height: 0,
            horizontal_tick_counter: 0,
            selected_element_index_in_on_display: 0
        };

        // init list with indicies and 0 as scroll status
        for i in 0..list.all_elements.items.len() {
            list.all_elements_scroll_status.push(ScrollStatus::new(i, 0));
        }

        return list;
    }

    pub fn next(&mut self) {
        self.all_elements.next();
    }

    pub fn previous(&mut self) {
        self.all_elements.previous();
    }

    pub fn get_elements(&mut self) -> &StatefulList<String> {
        &self.all_elements
    }

    // function used to change elements of the list
    // this function is nessecary becuase the scroll status 
    // and the on_display varirables have to be reset too
    pub fn change_elements(&mut self, new_elements : StatefulList<String>) {
        self.all_elements = new_elements;

        self.on_display = Vec::new();
        self.all_elements_scroll_status = Vec::with_capacity(self.all_elements.items.capacity());

        // reset list with indicies and 0 as scroll status
        for i in 0..self.all_elements.items.len() {
            self.all_elements_scroll_status.push(ScrollStatus::new(i, 0));
        }

    }

    pub fn get_on_display(&mut self) -> &Vec<String> {
        &self.on_display
    }

    // removes the text highlighting element from all list elements passed to the function
    pub fn remove_highlighting_element(&mut self, highlighting_element : char) {

        for i in 0..self.on_display.len() {
            if self.on_display[i].chars().nth(0).unwrap() == highlighting_element {
                self.on_display[i] = self.on_display[i][2..].to_string();
            }
        }

    }

    // adds the highlighting element to the currently selected list item
    pub fn add_highlighting_element(&mut self, highlighting_element : &str) {

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

        self.calc_vertical_scrolling(list_height); // calculate the vertical scrolling
        self.recover_scroll_status(); // because on_display has been reset the scroll_status of element in on_display has to be recovered
        self.calc_horizontal_scrolling(list_width, horizontal_scroll_delay); // calculated the horizontal scrolling

    }

    // recovers the scroll status of elements in on_display
    // this function presumes that the effective scroll_status has been lost
    // because e.g. on_display has been reset
    fn recover_scroll_status (&mut self) {
         
        // iterate over all visible elements
        for i in 0..self.on_display.len() {
            
            let mut scroll_string : String = self.all_elements.items[self.all_elements.state.offset+i].clone(); // get the current string to scroll
            let mut tmp_scroll_status = 0; // create tmp scroll status var that resembles the current scroll_status of the current scroll_string

            // repeat this loop as often as the element had been scrolled
            for j in 0..self.all_elements_scroll_status[self.all_elements.state.offset+i].scroll_status {
                let scroll_element_return_val = self.scroll_element(&scroll_string, tmp_scroll_status, 0, false); // call the scrolling function
                scroll_string = scroll_element_return_val.0; // set the string to its new value
                tmp_scroll_status = scroll_element_return_val.1; // update the temporary scroll status
                
            }

            self.on_display[i] = scroll_string; // set the recovered scrolled string
        }
    }

    // shift all characters in the string to the right 
    // (the leftmost character reappears on the right)
    fn scroll_element (&self, string : &String, scroll_status : u16, element_index : usize, reset_possible : bool) -> (String, u16) {
        
        // create a shifted string with the capacity of the regular string
        let mut shifted_string : String = String::with_capacity(string.capacity());

        // getting the index this way is nessecary because the string can be unicode
        let index_of_second_element = string.as_str().char_indices().map(|(i, _)| i).nth(1).unwrap();

        shifted_string.push_str(&string[index_of_second_element..]); // create string without first char
        shifted_string.push(string.as_str().chars().nth(0).unwrap()); // append original first element at the back

        let mut new_scroll_status : u16 = scroll_status; // this temporary variable is used as return value

        new_scroll_status += 1; // increment the scroll index

        // reset the scroll status if the scrolling has result in the original string
        if reset_possible && new_scroll_status as usize > self.all_elements.items[element_index].chars().count() {
            new_scroll_status = 0;
        }

        return (shifted_string, new_scroll_status); // return the new string and the new scroll status
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
        for i in 0..self.on_display.len() {

            // only do something if the text is longer than the space provided by the block
            // and the displayed string is longer that two characters
            if self.on_display[i].chars().count() > list_width && self.on_display[i].chars().count() > 2 {

                // call the scroll function
                let scroll_element_return_val = self.scroll_element(&self.on_display[i], // current element in on_display 
                     self.all_elements_scroll_status[self.all_elements.state.offset+i].scroll_status as u16, // scroll_status of the current element in on_display
                      self.all_elements_scroll_status[self.all_elements.state.offset+i].index, // index in all_elements of the current element in on_display
                       true); // check if the scrolling has resulted in the original string and if so reset the scroll_status

                self.all_elements_scroll_status[self.all_elements.state.offset+i].scroll_status = scroll_element_return_val.1; // set new scroll_status
                self.on_display[i] = scroll_element_return_val.0; // exchange old string for new one

            }
            else { // this block is called when the string shall not be scrolled

                // check if the string has been scrolled previously
                if self.all_elements_scroll_status[self.all_elements.state.offset+i].scroll_status > 0 {

                    // reset the string to its original value and reset the scroll_status
                    self.on_display[i] = self.all_elements.items[self.all_elements.state.offset+i].clone();
                    self.all_elements_scroll_status[self.all_elements.state.offset+i].scroll_status = 0;

                }

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
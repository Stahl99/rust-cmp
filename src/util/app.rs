use crate::util::StatefulList::StatefulList;
use crate::util::StatefulList::TabsState;
use crate::util::StatefulSelectedList::{
    StatefulSelectedList,
    CurrentElement
};

use tui::style::Color;

use argh::FromArgs;

pub struct App<'a> {
    pub horizontal_scroll_delay: u16,

    // used to hold the contents of the UI lists
    pub view_list: StatefulSelectedList,
    pub playlist_list: StatefulSelectedList,
    pub tracks_list: StatefulSelectedList,
    pub artist_list: StatefulSelectedList,
    pub albums_list: StatefulSelectedList,
    pub lengths_list: StatefulSelectedList,

    // These values are only for internal use
    pub track_name_list: StatefulSelectedList,
    pub artist_name_list: StatefulSelectedList,

    // the progress of the current song as ratio
    // has to be betwwen 0 and 1
    pub current_track_progress: f64, 
    pub track_progress_text: String, // string displayed in the progress bar

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

            view_list: StatefulSelectedList::new(vec![
                "Show Tracks".to_string(),
            ]),
            playlist_list: StatefulSelectedList::new(vec![
                "Ein sehr sehr langer String der unnötig lang ist".to_string(), "This documentation describes a number of methods and trait implementations on the char type. For technical reasons, there is additional, separate documentation in the std::char module as well.".to_string(), "Playlist 3".to_string(), "Playlist 4".to_string(), "Playlist 5".to_string(), 
                "Playlist 6".to_string(), "Playlist 7".to_string(), "Noch ein extraordinär langer String".to_string(), "Playlist 9".to_string(), "Playlist 10".to_string(), 
                "Playlist 11".to_string(), "Playlist 12".to_string(), "Playlist 13".to_string(), "Playlist 14".to_string(), "Playlist 15".to_string(), 
            ]),
            tracks_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), "Item1".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),
            artist_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), " ".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),
            albums_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), "Item1".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),
            lengths_list: StatefulSelectedList::new(vec![
                "Item0".to_string(), "Item1".to_string(), "Item2".to_string(), "Item3".to_string(), "Item4".to_string(),
                 "Item5".to_string(), "Item6".to_string(), "Item7".to_string(), "Item8".to_string(),
            ]),

            track_name_list: StatefulSelectedList::new(vec![
                "Test".to_string()
            ]),
            artist_name_list: StatefulSelectedList::new(vec![
                "Test".to_string()
            ]),

            current_track_progress: 0.5,
            track_progress_text: String::from("00 : 00"),
            
            current_element: CurrentElement::Playlists,
            // last element is empty so that it can be selected when no element of the tabs should be selected
            playbar_state: TabsState::new(vec!["<<", ">", ">>"]), 
            should_quit: false,

            up: false,
            down: false,
            left: false,
            right: false,

            header_color: Color::Rgb(216, 127, 26),
            title_color: Color::Rgb(0, 148, 255),
        };

        app.playbar_state.index = 3; // 3 = empty tab -> nothing is visibly selected

        // Select first element
        app.view_list.reset_selection();
        app.playlist_list.reset_selection();
        app.tracks_list.reset_selection();
        app.artist_list.reset_selection();
        app.albums_list.reset_selection();
        app.lengths_list.reset_selection();

        // set artist and track name
        app.set_track_name(String::from("Ein sehr sehr sehr langer Testtrackname"));
        app.set_artist_name(String::from("Ein sehr sehr sehr langer Artistname"));

        app.track_name_list.reset_selection();
        app.artist_name_list.reset_selection();

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

    pub fn set_track_name (&mut self, new_track_name : String) {

        self.track_name_list.change_elements(StatefulList::with_items(vec![new_track_name]));
    }

    pub fn set_artist_name (&mut self, new_artist_name : String) {
        self.artist_name_list.change_elements(StatefulList::with_items(vec![new_artist_name]));
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
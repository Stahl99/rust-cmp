use crate::util::app;
use crate::util::StatefulList::StatefulList;
use crate::util::StatefulSelectedList::CurrentElement;
use crate::player::Player;

pub struct PlayerInterface {
    music_player: Player,
    playlist_name: String,
}

impl PlayerInterface {

    pub fn new() -> PlayerInterface {
        PlayerInterface {
            music_player: Player::default(),
            playlist_name: "".to_string(),
        }
    }

    pub fn initialize (&mut self, app : &mut app::App) {
        let playlist_list = self.music_player.get_all_playlist_names();
        let playlist_stateful_list = StatefulList::with_items(playlist_list);
        app.playlist_list.change_elements(playlist_stateful_list);
    }
    
    // This function should be called on user input
    pub fn user_action (&mut self, app : &mut app::App) {
        let current_block = &app.current_element;

        // If playlist block is active, the tracks in the seleceted playlist are displayed
        if current_block.eq(&CurrentElement::Playlists) {
            self.playlist_name = app.playlist_list.get_selected_element().to_string();
            let track_list = self.music_player.get_all_titles_in_playlist(&self.playlist_name);
            let track_stateful_list = StatefulList::with_items(track_list);
            app.tracks_list.change_elements(track_stateful_list);
        // If main area is active, the playlist is loaded
        // into the queue and the selected song should be played
        } else if current_block.eq(&CurrentElement::MainArea) {
            let track_name = app.playlist_list.get_selected_element();
            self.music_player.clear_queue();
            self.music_player.load_playlist(&self.playlist_name);
            while self.music_player.get_current_song_title() != track_name.to_string() {
                self.music_player.next_song();
            }
            let title = track_name.to_string();
            app.set_track_name(title);
            self.music_player.play();
        // If playbar controls are active, send the user action
        // to the player
        } else if current_block.eq(&CurrentElement::Playbar) {
            match app.playbar_state.index {
                0 => self.music_player.prev_song(),
                1 => self.music_player.toggle_play_pause(),
                2 => self.music_player.next_song(),
                _ => {}
            }
        }
    }
}
use crate::util::app;
use crate::util::StatefulList::StatefulList;
use crate::util::StatefulSelectedList::CurrentElement;
use crate::player::Player;
use crate::player;

pub struct PlayerInterface {
    music_player: Player,
    playlist_name: String,
    playing: bool,
}

impl PlayerInterface {
    pub fn new() -> PlayerInterface {
        PlayerInterface {
            music_player: Player::default(),
            playlist_name: "".to_string(),
            playing: false,
        }
    }

    // Displays the playlists stored in the mpd server
    // Should be called once at application startup
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
            let songs_list = self.music_player.get_all_songs_in_playlist(&self.playlist_name);
            let len = songs_list.len();

            // Create vectors to store track data
            let mut albums_vec = Vec::<String>::with_capacity(len);
            let mut artists_vec = Vec::<String>::with_capacity(len);
            let mut duration_vec = Vec::<String>::with_capacity(len);

            // Fill the vectors with values retrieved from the player
            for i in 0..len-1 {
                albums_vec[i] = player::get_album_from_song(&songs_list[i]);
                artists_vec[i] = player::get_artist_from_song(&songs_list[i]);
                duration_vec[i] = PlayerInterface::transform_to_time_string(player::get_duration_from_song(&songs_list[i]));
            }
            let track_stateful_list = StatefulList::with_items(track_list);
            let albums_stateful_list = StatefulList::with_items(albums_vec);
            let artists_stateful_list = StatefulList::with_items(artists_vec);
            let durations_stateful_list = StatefulList::with_items(duration_vec);
            app.tracks_list.change_elements(track_stateful_list);
            app.albums_list.change_elements(albums_stateful_list);
            app.artist_list.change_elements(artists_stateful_list);
            app.lengths_list.change_elements(durations_stateful_list);
        }

        // If main area is active, the playlist is loaded
        // into the queue and the selected song should be played
        else if current_block.eq(&CurrentElement::MainArea) {
            let track_name = app.playlist_list.get_selected_element();
            self.music_player.clear_queue();
            self.music_player.load_playlist(&self.playlist_name);
            while self.music_player.get_current_song_title() != track_name.to_string() {
                self.music_player.next_song();
            }
            self.music_player.play();
            self.playing = true;
        }

        // If playbar controls are active, send the user action
        // to the player
        else if current_block.eq(&CurrentElement::Playbar) {
            match app.playbar_state.index {
                0 => {  
                    self.music_player.prev_song();
                },
                1 => {
                    self.music_player.toggle_play_pause();
                },
                2 => {
                    self.music_player.next_song();
                },
                _ => {}
            }
        }
    }

    // Updates the UI with playback information (Title, Artist, Playback Position)
    pub fn update_meta_display (&mut self, app: &mut app::App) {
        if self.playing {
            let song = self.music_player.get_current_song();
            app.set_artist_name(player::get_artist_from_song(&song));
            app.set_track_name(self.music_player.get_current_song_title());
            let duration = player::get_duration_from_song(&song);
            let fraction: f64 = 1.0 / duration as f64;
            app.current_track_progress = fraction * self.music_player.get_elapsed() as f64;
            app.track_progress_text = PlayerInterface::transform_to_time_string(self.music_player.get_elapsed());
        }
    }

    // Converts an integer value of seconds to a time string
    // * m:ss
    fn transform_to_time_string(seconds_input: i64) -> String {
        let mut seconds = seconds_input.clone();
        let mut minutes: i64 = 0;
        while seconds >= 60 {
            minutes += 1;
            seconds -= 60;
        }
        let time_string = minutes.to_string() + ":" + &seconds.to_string();
        return time_string;
    }
}
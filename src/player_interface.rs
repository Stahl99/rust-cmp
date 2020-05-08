use crate::util::app;
use crate::util::stateful_list::StatefulList;
use crate::util::stateful_selected_list::CurrentElement;
use crate::player::Player;
use crate::player;
use mpd::Song;

pub struct PlayerInterface {
    music_player: Player,
    playlist_name: String,
    playlist_length: u32,
    track_list: Vec<String>,
    songs_list: Vec<Song>,
    offset: usize,
}

impl PlayerInterface {
    pub fn new(ip_with_port: &String) -> PlayerInterface {
        PlayerInterface {
            music_player: Player::new(ip_with_port),
            playlist_name: "".to_string(),
            playlist_length: 0,
            track_list: Vec::<String>::new(),
            songs_list: Vec::<Song>::new(),
            offset: 0,
        }
    }

    // Displays the playlists stored in the mpd server
    // Should be called once at application startup
    pub fn initialize (&mut self, app : &mut app::App) {
        let playlist_list = self.music_player.get_all_playlist_names();
        let playlist_stateful_list = StatefulList::with_items(playlist_list);
        app.playlist_list.change_elements(playlist_stateful_list);
    }

    // cleanup connection
    pub  fn quit (&mut self) {
        self.music_player.close_conn();
    }
    
    // This function should be called on user input
    pub fn user_action (&mut self, app : &mut app::App) {
        let current_block = &app.current_element;

        // If playlist block is active, the tracks in the seleceted playlist are displayed
        if current_block.eq(&CurrentElement::Playlists) {
            self.playlist_name = app.playlist_list.get_selected_element().to_string();
            self.track_list = self.music_player.get_all_titles_in_playlist(&self.playlist_name);
            self.songs_list = self.music_player.get_all_songs_in_playlist(&self.playlist_name);
            self.playlist_length = self.songs_list.len() as u32;

            // Create vectors to store track data
            let mut albums_vec = Vec::<String>::with_capacity(self.playlist_length as usize);
            let mut artists_vec = Vec::<String>::with_capacity(self.playlist_length as usize);
            let mut duration_vec = Vec::<String>::with_capacity(self.playlist_length as usize);

            // Fill the vectors with values retrieved from the player
            for i in 0..self.playlist_length as usize {
                albums_vec.push(player::get_album_from_song(&self.songs_list[i]));
                artists_vec.push(player::get_artist_from_song(&self.songs_list[i]));
                duration_vec.push(PlayerInterface::transform_to_time_string(player::get_duration_from_song(&self.songs_list[i])));
            }

            // Save data in StatefulList
            let track_stateful_list = StatefulList::with_items(self.track_list.clone());
            let albums_stateful_list = StatefulList::with_items(albums_vec);
            let artists_stateful_list = StatefulList::with_items(artists_vec);
            let durations_stateful_list = StatefulList::with_items(duration_vec);

            // Change UI lists to display data in terminal
            app.tracks_list.change_elements(track_stateful_list);
            app.albums_list.change_elements(albums_stateful_list);
            app.artist_list.change_elements(artists_stateful_list);
            app.lengths_list.change_elements(durations_stateful_list);
        }

        // If main area is active, the playlist is loaded
        // into the queue and the selected song should be played
        else if current_block.eq(&CurrentElement::MainArea) {

            // Get the index of the selected track
            let selected_index = app.tracks_list.get_selected_index() as u32;
            // ...and save it as offset for later use
            self.offset = selected_index as usize;

            self.music_player.clear_queue();

            // Load new playlist in mpd server starting with the index
            self.music_player.load_playlist(&self.playlist_name, selected_index, self.playlist_length);

            self.music_player.play();
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
        if self.music_player.is_playing() {
            
            app.playbar_state.titles[1] = "‖‖";

            // Get the index of the currently playing track in the queue
            let index = self.music_player.get_current_song_id() as usize;

            // Access the stored metadata vectors with the offset, so the
            // data matches the currently playing track and set it in the terminal
            app.set_track_name(self.track_list[index + self.offset].clone());
            let song_object = self.songs_list[index + self.offset].clone();
            app.set_artist_name(player::get_artist_from_song(&song_object));

            // Calculation for progress bar
            let elapsed_seconds = self.music_player.get_elapsed();
            let progress = 1.0 / player::get_duration_from_song(&song_object) as f64 * elapsed_seconds as f64;
            app.current_track_progress = progress;
            app.track_progress_text = PlayerInterface::transform_to_time_string(elapsed_seconds);
        }
        else {
            app.playbar_state.titles[1] = ">>";
        }
    }

    // Converts an integer value of seconds to a time string
    // format "m:ss"
    fn transform_to_time_string(seconds_input: i64) -> String {
        let mut seconds = seconds_input.clone();
        let mut minutes: i64 = 0;

        // Add a minute for every sixty seconds
        while seconds >= 60 {
            minutes += 1;
            seconds -= 60;
        }

        // Add a "0" if there are less than 10 seconds
        let mut divider = ":";
        if seconds < 10 {
            divider = ":0";
        }
        
        let time_string = minutes.to_string() + divider + &seconds.to_string();
        return time_string;
    }
}
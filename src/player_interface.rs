use crate::util::app;
use crate::util::StatefulList::StatefulList;
use app::CurrentElement;
use crate::player::Player;

pub struct PlayerInterface {
    music_player: Player,
}

impl PlayerInterface {

    pub fn new() -> PlayerInterface {
        PlayerInterface {music_player: Player::default()}
    }

    pub fn initialize (&mut self, app : &mut app::App) {
        let playlist_list = self.music_player.get_all_playlist_names();
        let playlist_stateful_list = StatefulList::with_items(playlist_list);
        app.playlist_list.change_elements(playlist_stateful_list);
    }
    
    pub fn user_action (&mut self, app : &mut app::App) {
        let current_block = &app.current_element;

        if current_block.eq(&CurrentElement::Playlists) {
            let playlist_name = &app.playlist_list.get_selected_element();
            let track_list = self.music_player.get_all_titles_in_playlist(playlist_name.to_string());
            let track_stateful_list = StatefulList::with_items(track_list);
            app.albums_list.change_elements(track_stateful_list);
        }
    }
}
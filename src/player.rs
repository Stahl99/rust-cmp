extern crate mpd;

use mpd::client::*;
use std::net::{TcpStream, ToSocketAddrs};
use mpd::State;
use mpd::Song;
use mpd::Query;

pub struct Player {
    client: Client,
}

impl Default for Player {
    fn default() -> Player {
        Player::new(String::from("127.0.0.1:6600"))
    }
}

impl Player {
    pub fn new(addr: String) -> Player {
        Player {client: Client::connect(addr).unwrap()}
    }

    // creates a new connection to the given adress
    pub fn new_conn(& mut self, address: &String) {
        match Client::connect(address) {
            Ok(T) => self.client = T,
            Err(_) => println!("Error connecting to adress: {}", address),
        }
    }

    // starts playback on the server
    pub fn play(& mut self) {
        if self.client.status().unwrap().state == State::Play {
            println!("Already playing");
        }
        else {
            self.client.play().unwrap();
        }
    }

    // pauses playback if not already paused
    pub fn pause(& mut self) {
        if self.client.status().unwrap().state == State::Pause {
            println!("Already paused");
        }
        else {
            self.client.toggle_pause().unwrap();
        }
    }

    pub fn toggle_play_pause(& mut self) {
        if self.client.status().unwrap().state == State::Pause {
            self.play();
        }
        else if self.client.status().unwrap().state == State::Play {
            self.pause();
        }
    }

    // clears current queue
    pub fn clear_queue(&mut self) {
        self.client.clear().unwrap();
    }

    pub fn insert_as_first(&mut self, song: Song) {
        self.client.insert(song, 0).unwrap();
    }

    // add a song to a queue
    pub fn add_to_queue(& mut self, song: Song) {
        self.client.push(song).unwrap();
    }

    // get all names of all playlists
    pub fn get_all_playlist_names(& mut self) -> Vec<String> {
        let playlists = self.client.playlists().unwrap();
        let mut return_vec: Vec<String> = Vec::new();
        for playlist in playlists {
            return_vec.push(playlist.name)
        }

        return_vec
    }

    // get all titles in a specific playlist
    pub fn get_all_titles_in_playlist(& mut self, playlist_name: &String) -> Vec<String> {
        let songs = self.client.playlist(playlist_name).unwrap();
        let mut ret_songs: Vec<String> = Vec::new();
        for song in songs {
            ret_songs.push(song.title.unwrap());
        }

        ret_songs
    }

    // get all song objects in a playlist
    pub fn get_all_songs_in_playlist(&mut self, playlist_name: &String) -> Vec<Song> {
        self.client.playlist(playlist_name).unwrap()
    }

    // set playback volume
    pub fn set_volume(&mut self, volume: i8) {
        self.client.volume(volume).unwrap();
    }

    // switch current song to next song in queue
    pub fn next_song(&mut self) {
        self.client.next().unwrap(); 
    }

    // switch current song to prev song in queue
    pub fn prev_song(&mut self) {
        self.client.prev().unwrap();
    }

    // get vector of all songs in the current queue
    pub fn get_songs_in_queue(&mut self) -> Vec<String> {
        let songs = self.client.queue().unwrap();
        let mut ret_songs: Vec<String> = Vec::new();
        for song in songs {
            ret_songs.push(song.title.unwrap());
        }

        ret_songs
    }

    // seek to 'seconds' seconds in current song 
    pub fn seek(&mut self, seconds: i64) {
        self.client.rewind(seconds).unwrap();
    }

    // get title of current song
    pub fn get_current_song_title(&mut self) -> String {
        let song: Song = self.client.currentsong().unwrap().unwrap();
        let song_title: String;
        song_title = song.title.unwrap();
        
        song_title
    }

    // get all song objects in db 
    pub fn get_all_songs(&mut self, ) -> Vec<Song> {
        let songs = self.client.search(&Query::new(), (1, 2)).unwrap();

        songs
    }

    // get all song titles in db
    pub fn get_all_song_titles(&mut self) -> Vec<String> {
        let songs = self.client.search(&Query::new(), (1, 2)).unwrap();
        let mut song_titles: Vec<String> = Vec::new();
        for song in songs {
            song_titles.push(song.title.unwrap());
        }

        song_titles
    }

    // get Song object of current song
    pub fn get_current_song(&mut self) -> Song {
        self.client.currentsong().unwrap().unwrap()
    }

    // loads a playlist into the queue
    pub fn load_playlist (&mut self, playlist_name: &String) {
        self.client.load(playlist_name, ..).unwrap();
    }
}

// get a String containing the album from a song object
pub fn get_album_from_song(song: &Song) -> String {
    if song.tags.get("album").is_some() {
        let album: &String = song.tags.get("album").unwrap();
        album.to_owned()
    }
    else {
        String::from("unknown album")
    }
    
}

// get a String containing the artist from a song object
pub fn get_artist_from_song(song: &Song) -> String {
    if song.tags.get("artist").is_some() {
        let artist: &String = song.tags.get("artist").unwrap();
        artist.to_owned()
    }
    else {
        String::from("unknwon artist")
    }
}

// get the duration of the song in seconds
pub fn get_duration_from_song(song: &Song) -> i64 {
    song.duration.unwrap().num_seconds()
}
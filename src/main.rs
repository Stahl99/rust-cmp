mod audio_player;

fn main() {
    let mut input = String::new();
    println!("Hello, world!");
    let sink = audio_player::play_music();
    std::io::stdin().read_line(&mut input);
}

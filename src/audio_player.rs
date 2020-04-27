use std::fs::File;
use std::io::BufReader;
use rodio::Sink;


pub fn play_music() -> rodio::Sink
{
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open("test.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    return sink;
}


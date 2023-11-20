extern crate sdl2;
// extern  crate gl;

use std::path::Path;

use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, Channel};

// Audio -> play .wav, .ogg
    // Volume
    // Play
    // Stop
    // Loop
const CHUNK_SIZE: i32 = 1024;
const FREQUENCY: i32 = 44_100;

pub struct AudioPlayer {
    pub mixer_context: sdl2::mixer::Sdl2MixerContext,
}

#[allow(dead_code)]
impl AudioPlayer {
    pub fn new(numchans: i32, sdl_context: sdl2::Sdl) -> Self {
        //let sdl_context = sdl2::init().unwrap();
        let _audio = sdl_context.audio().unwrap();
        let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3 | sdl2::mixer::InitFlag::FLAC | sdl2::mixer::InitFlag::MOD | sdl2::mixer::InitFlag::OGG).unwrap();

        sdl2::mixer::open_audio(FREQUENCY, AUDIO_S16LSB, DEFAULT_CHANNELS, CHUNK_SIZE).unwrap();
        sdl2::mixer::allocate_channels(numchans);

        AudioPlayer { mixer_context }
    }

    pub fn play(&self, file_path: &Path, loops: i32, volume: i32) -> Channel {
        let audio_chunk = sdl2::mixer::Chunk::from_file(file_path).unwrap();
        let channel = sdl2::mixer::Channel::all().play(&audio_chunk, loops).unwrap();
        channel.set_volume(volume);
        channel
    }

    pub fn pause(&self, channel: Channel) {
        channel.pause();
    }

    pub fn resume(&self, channel: Channel) {
        channel.resume();
    }

    pub fn stop(&self, channel: Channel) {
        channel.halt();
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        sdl2::mixer::Music::hook_finished(|| println!("Music is ended"))
    }
}
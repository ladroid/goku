extern crate sdl2;
// extern  crate gl;

use std::path::Path;

use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};

// Audio -> play .wav, .ogg
    // Volume
    // Play
    // Stop
    // Loop
const CHUNK_SIZE: i32 = 1024;
const FREQUENCY: i32 = 44_100;

pub struct AudioPlayer {
    pub audio: sdl2::AudioSubsystem,
    pub mixer_context: sdl2::mixer::Sdl2MixerContext,
    pub timer: sdl2::TimerSubsystem,
}

#[allow(dead_code)]
impl AudioPlayer {
    pub fn new(numchans: i32, sdl_context: sdl2::Sdl) -> Self {
        // let sdl_context = sdl2::init().unwrap();
        let _audio = sdl_context.audio().unwrap();
        let timer = sdl_context.timer().unwrap();

        sdl2::mixer::open_audio(FREQUENCY, AUDIO_S16LSB, DEFAULT_CHANNELS, CHUNK_SIZE).unwrap();
        let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3 | sdl2::mixer::InitFlag::FLAC | sdl2::mixer::InitFlag::MOD | sdl2::mixer::InitFlag::OGG).unwrap();
        sdl2::mixer::allocate_channels(numchans);

        AudioPlayer { audio: _audio, mixer_context, timer }
    }

    pub fn play(&mut self, file_path: &Path, loops: i32, volume: i32, delay: u32) -> sdl2::mixer::Music {
        let music = sdl2::mixer::Music::from_file(file_path).unwrap();
        println!("music => {:?}", music);
        println!("music type => {:?}", music.get_type());
        sdl2::mixer::Music::set_volume(volume);
        println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
        println!("play => {:?}", music.play(loops));
        self.timer.delay(delay);
        music
    }

    pub fn pause(&self) {
        sdl2::mixer::Music::pause();
    }

    pub fn resume(&self) {
        sdl2::mixer::Music::resume();
    }

    pub fn stop(&self) {
        sdl2::mixer::Music::halt();
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        sdl2::mixer::Music::hook_finished(|| println!("Music is ended"))
    }
}
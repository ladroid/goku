use goku::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set current directory to the root of the project
    std::env::set_current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to set project root as current directory");

    let mut window = two_d::Window::new("Test", 800, 600, false)?;
    let mut audio = two_d::audio::AudioPlayer::new(4);
    audio.play(std::path::Path::new("test_assets/Dragon-Mystery.ogg"), -1, 35);
    window.canvas.present();

    Ok(())
}
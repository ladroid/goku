mod two_d;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = two_d::Window::new("Test", 800, 600)?;
    let mut audio = two_d::AudioPlayer::new(4, window.sdl_context);
    audio.play(std::path::Path::new("Dragon-Mystery.ogg"), 0, 10, 5_000_000);
    window.canvas.present();

    Ok(())
}
mod audio;

use crate::audio::MusicPlayer;

fn play_music() -> Result<(), Box<dyn std::error::Error>> {
    let music = MusicPlayer::new()?;

    music.play_file("assets/example.mp3")?;
    music.sleep_until_end();

    Ok(())

}

fn main() {
    play_music();
}


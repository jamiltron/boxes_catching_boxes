extern crate sdl2_mixer;

use sdl2_mixer::Music;

pub struct AudioManager {
    catch_sound: Music,
    crash_sound: Music,
    muted: bool,
}

impl AudioManager {
    pub fn new(catch_sound: Music, crash_sound: Music) -> AudioManager {
        AudioManager {
            catch_sound: catch_sound,
            crash_sound: crash_sound,
            muted: false,
        }
    }

    pub fn play_catch(&self) -> () {
        if !self.muted {
            self.catch_sound.play(1).unwrap();
        }
    }

    pub fn play_crash(&self) -> () {
        if !self.muted {
            self.crash_sound.play(1).unwrap();
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }
}

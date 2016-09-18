use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    pub left_is_down: bool,
    pub right_is_down: bool,
    pub mute_pressed: bool,
    pub should_quit: bool,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            left_is_down: false,
            right_is_down: false,
            mute_pressed: false,
            should_quit: false,
        }
    }

    pub fn handle_input(&mut self, event_pump: &mut EventPump) -> () {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.should_quit = true;
                    return;
                }
                Event::KeyDown { keycode: Some(Keycode::M), .. } => self.mute_pressed = true,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => self.left_is_down = true,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => self.right_is_down = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => self.left_is_down = false,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => self.right_is_down = false,
                _ => {}
            }
        }
    }
}

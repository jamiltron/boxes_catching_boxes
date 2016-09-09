extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

pub struct Entity {
    pub rect: Rect,
    pub color: Color,
}

impl Entity {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Entity {
        Entity {
            rect: Rect::new(x, y, width, height),
            color: color,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) -> () {
        renderer.set_draw_color(self.color);
        match renderer.fill_rect(self.rect) {
            Ok(..) => (),
            Err(e) => println!("Error rendering game_rect: {}", e),
        }
    }
}

extern crate sdl2;

use entity::Entity;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

#[derive(Copy, Clone)]
pub struct Pickup {
    pub entity: Entity,
    pub drop_speed: f32,
    pub enabled: bool,
}

impl Pickup {
    pub fn new(width: u32, height: u32, color: Color, drop_speed: f32) -> Self {
        Pickup {
            entity: Entity::new(0, 0, width, height, color),
            drop_speed: drop_speed,
            enabled: false,
        }
    }

    pub fn enable(&mut self, x: i32, y: i32) -> () {
        self.enabled = true;
        self.entity.rect.set_x(x);
        self.entity.rect.set_y(y);
    }

    pub fn disable(&mut self) -> () {
        self.enabled = false;
    }

    pub fn update(&mut self, dt: f32) -> () {
        if self.enabled {
            let y = self.entity.rect.y();
            self.entity.rect.set_y(y + (self.drop_speed * dt) as i32);
        }
    }

    pub fn render(&self, renderer: &mut Renderer) -> () {
        if self.enabled {
            self.entity.render(renderer);
        }
    }
}

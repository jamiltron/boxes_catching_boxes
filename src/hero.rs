extern crate sdl2;

use entity::Entity;
use input_manager::InputManager;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub struct Hero {
    pub entity: Entity,
    pub move_speed: f32,
}

impl Hero {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color, move_speed: f32) -> Hero {
        Hero {
            entity: Entity::new(x, y, width, height, color),
            move_speed: move_speed,
        }
    }

    pub fn clamp(&mut self, min: i32, max: i32) -> () {
        let x = self.entity.rect.x();
        let w = self.entity.rect.width();

        if x < min {
            self.entity.rect.set_x(min);
        }

        if x + w as i32 > max {
            self.entity.rect.set_x(max - w as i32);
        }
    }

    pub fn render(&self, renderer: &mut Renderer) -> () {
        self.entity.render(renderer);
    }

    pub fn update(&mut self, input_manager: &InputManager, dt: f32) -> () {
        let x = self.entity.rect.x();
        if input_manager.left_is_down && !input_manager.right_is_down {
            self.entity.rect.set_x(x - (self.move_speed * dt) as i32);
        } else if input_manager.right_is_down && !input_manager.left_is_down {
            self.entity.rect.set_x(x + (self.move_speed * dt) as i32);
        }
    }
}

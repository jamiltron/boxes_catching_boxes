extern crate rand;

use drop::Drop;
use rand::Rng;
use rand::ThreadRng;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

const DROP_WIDTH: u32 = 16;
const DROP_HEIGHT: u32 = 16;

pub struct Dropper {
    drops: [Drop; 12],
    drop_time: f32,
    current_time: f32,
    x_max: i32,
    y_max: i32,
    rng: ThreadRng,
}

impl Dropper {
    pub fn new(x_max: i32, y_max: i32, drop_time: f32, color: Color, drop_speed: f32) -> Dropper {
        Dropper {
            drops: [Drop::new(DROP_WIDTH, DROP_HEIGHT, color, drop_speed); 12],
            drop_time: drop_time,
            current_time: 0.,
            x_max: x_max,
            y_max: y_max,
            rng: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
        self.current_time += dt;

        if self.current_time >= self.drop_time {
            self.current_time -= self.drop_time;

            for drop in self.drops.iter_mut() {
                if drop.enabled == false {
                    let y: i32 = 0 - drop.entity.rect.height() as i32;
                    let x: i32 =
                        self.rng.gen_range(0, (self.x_max as u32 / DROP_WIDTH) + 1) as i32 *
                        DROP_WIDTH as i32;
                    drop.enable(x, y);
                    break;
                }
            }
        }

        for drop in self.drops.iter_mut() {
            drop.update(dt);
            if drop.entity.rect.y() >= self.y_max {
                drop.disable();
            }
        }
    }

    pub fn check_overlap(&mut self, catch_rect: Rect) -> i32 {
        let mut score = 0;
        for drop in self.drops.iter_mut() {
            if drop.enabled == true {
                if drop.entity.rect.has_intersection(catch_rect) {
                    score += 1;
                    drop.disable()
                }
            }
        }
        score
    }

    pub fn render(&self, renderer: &mut Renderer) -> () {
        for &drop in self.drops.iter() {
            drop.render(renderer);
        }
    }
}

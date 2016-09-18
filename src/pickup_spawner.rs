extern crate rand;

use pickup::Pickup;
use rand::Rng;
use rand::ThreadRng;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

const PICKUP_WIDTH: u32 = 16;
const PICKUP_HEIGHT: u32 = 16;

pub struct PickupSpawner {
    pickups: [Pickup; 12],
    drop_time: f32,
    current_time: f32,
    x_max: i32,
    y_max: i32,
    rng: ThreadRng,
}

impl PickupSpawner {
    pub fn new(x_max: i32, y_max: i32, drop_time: f32, color: Color, drop_speed: f32) -> Self {
        PickupSpawner {
            pickups: [Pickup::new(PICKUP_WIDTH, PICKUP_HEIGHT, color, drop_speed); 12],
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

            for pickup in self.pickups.iter_mut() {
                if pickup.enabled == false {
                    let y: i32 = 0 - pickup.entity.rect.height() as i32;
                    let x: i32 =
                        self.rng.gen_range(0, (self.x_max as u32 / PICKUP_WIDTH) + 1) as i32 *
                        PICKUP_WIDTH as i32;
                    pickup.enable(x, y);
                    break;
                }
            }
        }

        for pickup in self.pickups.iter_mut() {
            pickup.update(dt);
            if pickup.entity.rect.y() >= self.y_max {
                pickup.disable();
            }
        }
    }

    pub fn check_overlap(&mut self, catch_rect: Rect) -> i32 {
        let mut score = 0;
        for pickup in self.pickups.iter_mut() {
            if pickup.enabled == true {
                if pickup.entity.rect.has_intersection(catch_rect) {
                    score += 1;
                    pickup.disable()
                }
            }
        }
        score
    }

    pub fn render(&self, renderer: &mut Renderer) -> () {
        for &pickup in self.pickups.iter() {
            pickup.render(renderer);
        }
    }
}

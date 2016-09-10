extern crate sdl2;
extern crate rand;

mod drop;
mod dropper;
mod entity;
mod game_timer;
mod hero;
mod input_manager;

use dropper::Dropper;
use entity::Entity;
use game_timer::GameTimer;
use hero::Hero;
use input_manager::InputManager;
use sdl2::pixels::Color;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 480;
const ENTITY_HEIGHT: u32 = 32;
const ENTITY_WIDTH: u32 = 32;

fn main() {

    // create the various subsystems and contexts, etc.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let window = video_subsystem.window("Boxes Catching Boxes", GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer = window.renderer().present_vsync().build().unwrap();

    // create our own manager classes
    let mut game_timer = GameTimer::new(timer_subsystem.performance_counter());
    let mut input_manager = InputManager::new();
    let mut dropper = Dropper::new(GAME_WIDTH as i32,
                                   GAME_HEIGHT as i32,
                                   1000.0,
                                   Color::RGB(255, 255, 255),
                                   0.1);

    // create our game entities
    let grass = Entity::new(0,
                            (GAME_HEIGHT - ENTITY_HEIGHT) as i32,
                            GAME_WIDTH,
                            ENTITY_HEIGHT,
                            Color::RGB(140, 207, 127));

    let mut hero = Hero::new((GAME_WIDTH / 2 - ENTITY_WIDTH / 2) as i32,
                             (GAME_HEIGHT - ENTITY_HEIGHT * 2) as i32,
                             ENTITY_WIDTH,
                             ENTITY_HEIGHT,
                             Color::RGB(255, 102, 102),
                             0.5);

    // main game loop
    'running: loop {
        // grab input
        input_manager.handle_input(&mut event_pump);
        if input_manager.should_quit {
            break 'running;
        }

        // update everything
        game_timer.update(timer_subsystem.performance_counter(),
                          timer_subsystem.performance_frequency());
        hero.update(&input_manager, game_timer.dt);
        hero.clamp(0, GAME_WIDTH as i32);
        dropper.update(game_timer.dt);
        dropper.check_caught(hero.entity.rect);

        // render everything
        renderer.set_draw_color(Color::RGB(101, 156, 239));
        renderer.clear();
        dropper.render(&mut renderer);
        grass.render(&mut renderer);
        hero.render(&mut renderer);
        renderer.present();
    }
}

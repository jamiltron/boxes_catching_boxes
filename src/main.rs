extern crate rand;
extern crate sdl2;
extern crate sdl2_mixer;
extern crate sdl2_ttf;

mod audio_manager;
mod drop;
mod dropper;
mod entity;
mod game_timer;
mod hero;
mod input_manager;
mod score_keeper;

use audio_manager::AudioManager;
use dropper::Dropper;
use entity::Entity;
use game_timer::GameTimer;
use hero::Hero;
use input_manager::InputManager;
use score_keeper::ScoreKeeper;
use sdl2::pixels::Color;
use sdl2_mixer::AUDIO_S16LSB;
use std::path::Path;

const CHANNELS: isize = 2;
const CHUNK_SIZE: isize = 1024;
const DROP_TIME: f32 = 1000.0;
const DROP_SPEED: f32 = 0.1;
const FREQUENCY: isize = 44100;
const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 480;
const HERO_SPEED: f32 = 0.5;
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

    // audio stuff
    let _ = sdl_context.audio().unwrap();
    let _ = sdl2_mixer::open_audio(FREQUENCY, AUDIO_S16LSB, CHANNELS, CHUNK_SIZE);
    sdl2_mixer::allocate_channels(0);
    let catch_sound = sdl2_mixer::Music::from_file(Path::new("res/audio/get.wav")).unwrap();
    let crash_sound = sdl2_mixer::Music::from_file(Path::new("res/audio/crash.wav")).unwrap();

    // create our own manager classes
    let mut audio_manager = AudioManager::new(catch_sound, crash_sound);
    let mut game_timer = GameTimer::new(timer_subsystem.performance_counter());
    let mut input_manager = InputManager::new();

    // font rendering
    let ttf_context = sdl2_ttf::init().unwrap();
    let font = ttf_context.load_font(Path::new("res/fonts/kenpixel_mini.ttf"), 16).unwrap();
    let mut score_keeper = ScoreKeeper::new(font, Color::RGB(050, 050, 050), &renderer, GAME_WIDTH);


    // create our game entities
    let mut dropper = Dropper::new(GAME_WIDTH as i32,
                                   GAME_HEIGHT as i32,
                                   DROP_TIME,
                                   Color::RGB(255, 255, 153),
                                   DROP_SPEED);

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
                             HERO_SPEED);

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

        if input_manager.mute_pressed {
            println!("toggling mute!");
            audio_manager.toggle_mute();
            input_manager.mute_pressed = false;
        }

        hero.update(&input_manager, game_timer.dt);
        hero.clamp(0, GAME_WIDTH as i32);
        dropper.update(game_timer.dt);

        // update score stuff
        let catch_points = dropper.check_overlap(hero.entity.rect);
        if catch_points > 0 {
            audio_manager.play_catch();
        }
        let crash_points = dropper.check_overlap(grass.rect);
        if crash_points > 0 {
            audio_manager.play_crash();
        }
        let points = catch_points - crash_points;
        score_keeper.update(points);


        // render everything
        renderer.set_draw_color(Color::RGB(101, 156, 239));
        renderer.clear();
        dropper.render(&mut renderer);
        grass.render(&mut renderer);
        hero.render(&mut renderer);
        score_keeper.render(&mut renderer);
        renderer.present();
    }
}

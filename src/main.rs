mod display;
mod game;
mod tetromino;
use display::*;
use game::Game_State;
use game::Input_State;
use game::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::*;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "TETRIS",
            WIDTH as u32 * GRID_SIZE as u32,
            HEIGHT as u32 * GRID_SIZE as u32 + 60,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let font_path = "assets/Gilroy-Light.ttf";
    let font_size = 24;
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game: Game_State = Game_State::new();
    let mut input: Input_State = Input_State::new();
    'running: loop {
        game.set_time((sdl_context.timer().unwrap().ticks() as f32) / 1000.0);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let keyboard_state = event_pump.keyboard_state();
        let prev_input: Input_State = input;
        input.left = keyboard_state.is_scancode_pressed(Scancode::Left) as u8;
        input.right = keyboard_state.is_scancode_pressed(Scancode::Right) as u8;
        input.up = keyboard_state.is_scancode_pressed(Scancode::Up) as u8;
        input.down = keyboard_state.is_scancode_pressed(Scancode::Down) as u8;
        input.a = keyboard_state.is_scancode_pressed(Scancode::Space) as u8;

        input.dleft = input.left as i8 - prev_input.left as i8;
        input.dright = input.right as i8 - prev_input.right as i8;
        input.dup = input.up as i8 - prev_input.up as i8;
        input.ddown = input.down as i8 - prev_input.down as i8;
        input.da = input.a as i8 - prev_input.a as i8;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        game.update(&input);
        render_game(&mut game, &mut canvas, &font);
        canvas.present();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

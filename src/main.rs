use std::{time::Duration, thread::sleep};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().expect("SDL Initialization Failed.");

    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL Video Subsystem");

    let window = video_subsystem
        .window("Rust Tetris Game", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas.");

    canvas.set_draw_color(Color::RGB(255,120,255));
    canvas.clear();
    canvas.present();

    let mut event_bump = sdl_context.event_pump().expect("Failed to get SDL event bump");

    'running: loop {
        for event in event_bump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                _ => {}
            }
            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

}

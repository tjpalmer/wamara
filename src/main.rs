extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, SystemTime};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 1280, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let _texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut frame_seconds = 0.0;
    let mut last_time = SystemTime::now();
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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
        // The rest of the game loop goes here...

        canvas.present();

        let time = SystemTime::now()
            .duration_since(last_time)
            .unwrap()
            .as_nanos() as i128;
        let period = 1_000_000_000i128 / 60;
        let remaining = if time > period { 0 } else { period - time };
        ::std::thread::sleep(Duration::new(0, remaining as u32));

        let next_time = SystemTime::now();
        let duration = next_time.duration_since(last_time).unwrap();
        last_time = next_time;
        let memory = 0.95;
        frame_seconds = memory * frame_seconds + (1.0 - memory) * duration.as_secs_f64();
        if i == 0 {
            println!("{}", 1.0 / frame_seconds);
        }
    }
}

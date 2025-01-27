//use lib;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);

    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, // Exit on quit event

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running, // Exit on Escape key

                _ => {}
            }
        }
        ::std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

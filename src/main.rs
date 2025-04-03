extern crate sdl2;
mod vectors;
use vectors::Vector;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;


fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem
        .window("3D Triangle Rasterizer", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Prostor za definicijo vektorjev

    //


    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Clear the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Tukaj narišemo oblike s pomočjo funkcij definiranih da rišejo na canvas(glejta v test.rs)

        //


        // Present the rendered frame
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
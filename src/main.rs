extern crate sdl2;
mod vectors;
use sdl2::keyboard::Keycode;
use vectors::Vector;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;


fn draw_gradient_background(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, width: i32, height: i32) {
    for y in 0..height {
        let t = y as f32 / height as f32;
        let r = (255.0 * (1.0 - t)) as u8; // Red fades from 255 to 0
        let g = (255.0 * t) as u8;         // Green fades from 0 to 255
        let b = 255; // Constant blue for a blue-tinted gradient

        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.draw_line((0, y), (width, y)).unwrap();
    }
}

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
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                /*Event::KeyDown {keycode: Some(Keycode::X), .. } => {
                    canvas.set_draw_color(Color::RGB(0, 255, 255));
                    canvas.present();
                }*/
                _ => {}
            }
        }

        // Background
        draw_gradient_background(&mut canvas, 800, 600);
        // Tukaj narišemo oblike s pomočjo funkcij definiranih da rišejo na canvas(glejta v test.rs)

        // Present the rendered frame
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
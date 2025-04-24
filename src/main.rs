use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod vectors;
use vectors::Vector;

mod scene;
use scene::{ColorType, Light, Material, Scene, Sphere};

mod camera;
use camera::Camera;

mod tests;

const WIDTH: u16 = 1400;
const ASPECT_RATIO: (u16, u16) = (16, 10);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;
    let (x_ratio, y_ratio) = ASPECT_RATIO;
    let height = (WIDTH as f64 * (y_ratio as f64 / x_ratio as f64)) as u32;
    let window = video
        .window("Ray Tracing in SDL2", WIDTH as u32, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let scene = &tests::SCENE1;

    let camera = Camera::new(
        Vector {
            x: -3.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    );

    camera.draw(&mut canvas, &scene, WIDTH, ASPECT_RATIO);
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::NUM_1),
                    ..
                } => {
                    camera.draw(&mut canvas, &tests::SCENE1, WIDTH, ASPECT_RATIO);
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::NUM_2),
                    ..
                } => {
                    camera.draw(&mut canvas, &tests::SCENE2, WIDTH, ASPECT_RATIO);
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::NUM_3),
                    ..
                } => {
                    camera.draw(&mut canvas, &tests::SCENE3, WIDTH, ASPECT_RATIO);
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::NUM_4),
                    ..
                } => {
                    camera.draw(&mut canvas, &tests::SCENE4, WIDTH, ASPECT_RATIO);
                    canvas.present();
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

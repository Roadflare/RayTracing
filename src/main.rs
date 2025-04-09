extern crate sdl2;
mod vectors;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use vectors::Vector;
mod camera_and_rays_and_scene;
use camera_and_rays_and_scene::{Camera, Scene, Sphere};

const WIDTH: u16 = 1400;
const RATIO: (u16, u16) = (16, 10);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;
    let (x_ratio, y_ratio) = RATIO;
    let height = (WIDTH as f64 * (y_ratio as f64 / x_ratio as f64)) as u32;
    let window = video
        .window("Ray Tracing in SDL2", WIDTH as u32, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let scene = Scene {
        spheres: vec![Sphere {
            center: Vector {
                x: 0.0,
                y: 0.0,
                z: 3.0,
            },
            radius: 1.0,
        }],
    };
    //definiramo kamero
    let camera = Camera::new(
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    camera.draw(&mut canvas, &scene, WIDTH, RATIO);
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
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

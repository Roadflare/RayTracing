use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod vectors;
use vectors::Vector;

mod scene;

mod camera;
use camera::Camera;

mod tests;

const WIDTH: u16 = 1000;
const ASPECT_RATIO: (u16, u16) = (16, 10);
const GLOBINA: u32 = 5;

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

    let mut scene = &tests::SCENE1;

    let mut camera = Camera::new(Vector::make(-3.0, 0.0, 0.0), Vector::make(1.0, 0.0, 0.0));

    camera.draw(&mut canvas, &scene, WIDTH, ASPECT_RATIO, GLOBINA);
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit{ .. } = event {
                break 'running;
            }
            if let Event::KeyDown {
                keycode: Some(key), ..
            } = event
            {
                match key {
                    Keycode::Escape => break 'running,

                    Keycode::Num1 => scene = &tests::SCENE1,
                    Keycode::Num2 => scene = &tests::SCENE2,
                    Keycode::Num3 => scene = &tests::SCENE3,
                    Keycode::Num4 => scene = &tests::SCENE4,
                    Keycode::Num5 => scene = &tests::SCENE5,
                    Keycode::Num6 => scene = &tests::SCENE6,
                    Keycode::H => scene = &tests::SCENE_H,
                    Keycode::J => scene = &tests::SCENE_J,
                    Keycode::K => scene = &tests::PEAK_K,

                    Keycode::W
                    | Keycode::A
                    | Keycode::S
                    | Keycode::D
                    | Keycode::Space
                    | Keycode::LShift => {
                        camera = camera.relocate(key);
                    }
                    Keycode::Q => {
                        camera = camera.rotate(-15.0);
                    }
                    Keycode::E => {
                        camera = camera.rotate(15.0);
                    }
                    Keycode::C => {
                        camera = camera.reset_location();
                    }
                    Keycode::V => {
                        camera = camera.reset_location_reversed();
                    }
                    _ => {}
                }

                camera.draw(&mut canvas, &scene, WIDTH, ASPECT_RATIO, GLOBINA);
                canvas.present();
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

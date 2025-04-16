use crate::vectors::Vector;
use sdl2::pixels::Color;

use crate::scene::Scene;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

pub struct Camera {
    pub coords: Vector,
    pub direction: Vector,
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }
}

impl Camera {
    pub fn new(coords: Vector, direction: Vector) -> Self {
        Camera {
            coords,
            direction: direction.normalized(),
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        scene: &Scene,
        width: u16,
        ratio: (u16, u16),
    ) {
        let up = Vector::make(0.0, 1.0, 0.0);
        let forward = self.direction;
        let right = forward.cross(&up).normalized();
        let true_up = right.cross(&forward).normalized();

        let (x_ratio, y_ratio) = ratio;
        let height = (width as f64 * (y_ratio as f64 / x_ratio as f64)) as u16;
        let aspect_ratio = width as f64 / height as f64;

        for y in 0..height {
            for x in 0..width {
                let u = (x as f64 + 0.5) / width as f64 - 0.5;
                let v = (y as f64 + 0.5) / height as f64 - 0.5;

                let pixel_dir = forward + right * (u * 2.0 * aspect_ratio) + true_up * (-v * 2.0);

                let ray = Ray::new(self.coords, pixel_dir);

                let color = if let Some((sphere, point)) = scene.trace_ray(&ray) {
                    let normal = sphere.normal(point);

                    let mut brightness = 0.0;

                    for light in &scene.lights {
                        let light_dir = (light.position - point).normalized();
                        let dot = normal.dot(&light_dir).max(0.0);
                        brightness += dot * light.intensity;
                    }

                    brightness = brightness.min(1.0);

                    let r = (255.0 * brightness) as u8;
                    let g = (30.0 * brightness) as u8;
                    let b = (30.0 * brightness) as u8;

                    Color::RGB(r, g, b)
                } else {
                    Color::RGB(150, 170, 150 + (y as f64 * 105. / height as f64) as u8)
                };

                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}

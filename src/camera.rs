use crate::ColorType;
use crate::scene::Scene;
use crate::vectors::Vector;
use sdl2::pixels::Color;

pub struct Ray {
    pub origin: Vector,
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

pub struct Camera {
    ///Location of camera/where all rays shoot
    pub coords: Vector,
    /// Direction that camera faces
    pub direction: Vector,
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
        let cam_direction = self.direction;
        let right = cam_direction.cross(&up).normalized();
        let true_up = right.cross(&cam_direction).normalized();

        let (x_ratio, y_ratio) = ratio;
        let height = (width as f64 * (y_ratio as f64 / x_ratio as f64)) as u16;
        let aspect_ratio = x_ratio as f64 / y_ratio as f64;

        for y_pixel_coordinate in 0..height {
            for x_pixel_coordinate in 0..width {
                let u = (x_pixel_coordinate as f64 + 0.5) / width as f64 - 0.5;
                let v = (y_pixel_coordinate as f64 + 0.5) / height as f64 - 0.5;

                let pixel_dir = cam_direction + right * (u * 2.0 * aspect_ratio) + true_up * (-v * 2.0);

                let ray = Ray::new(self.coords, pixel_dir);

                let color = if let Some((sphere, point)) = scene.trace_ray(&ray) {
                    let normal = sphere.normal(point);

                    let mut brightness = 0.0;
                    for light in &scene.lights {
                        let light_dir = (light.position - point).normalized();
                        brightness += normal.dot(&light_dir).max(0.0) * light.intensity;
                    }
                    brightness = brightness.min(1.0);

                    let base_color = match &sphere.material.color {
                        ColorType::Solid(c) => *c,
                        ColorType::Function(f) => f(point),
                    };

                    let r = (base_color.r as f64 * brightness) as u8;
                    let g = (base_color.g as f64 * brightness) as u8;
                    let b = (base_color.b as f64 * brightness) as u8;

                    Color::RGB(r, g, b)
                } else {
                    Color::RGB(150, 170, 150 + (y_pixel_coordinate as f64 * 105. / height as f64) as u8)
                };

                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x_pixel_coordinate as i32, y_pixel_coordinate as i32));
            }
        }
    }
}

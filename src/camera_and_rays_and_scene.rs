use crate::vectors::Vector;
use sdl2::pixels::Color;

pub struct Camera {
    coords: Vector,
    direction: Vector,
}

impl Camera {
    pub fn new(coords: Vector, direction: Vector) -> Self {
        Camera {
            coords,
            direction: direction.norm(),
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        scene: &Scene,
        width: u16,
        ratio: (u16, u16),
    ) {
        let up = Vector { x: 0.0, y: 1.0, z: 0.0 };
        let forward = self.direction;
        let right = forward.cross(&up).norm();
        let true_up = right.cross(&forward).norm();

        let (x_ratio, y_ratio) = ratio;
        let height = (width as f64 * (y_ratio as f64 / x_ratio as f64)) as u16;
        let aspect_ratio = width as f64 / height as f64;

        for y in 0..height {
            for x in 0..width {
                let u = (x as f64 + 0.5) / width as f64 - 0.5;
                let v = (y as f64 + 0.5) / height as f64 - 0.5;

                let pixel_dir = forward
                    + right * (u * 2.0 * aspect_ratio)
                    + true_up * (-v * 2.0);

                let ray = Ray::new(self.coords, pixel_dir);

                let color = if scene.trace_ray(&ray) {
                    Color::RGB(255, 0, 0)
                } else {
                    Color::RGB(0, 0, 0)
                };

                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}

pub struct Ray {
    location: Vector,
    direction: Vector,
}

impl Ray {
    fn new(location: Vector, direction: Vector) -> Self {
        Ray {
            location,
            direction: direction.norm(),
        }
    }
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    fn trace_ray(&self, ray: &Ray) -> bool {
        self.spheres.iter().any(|s: &Sphere| s.hit(ray))
    }
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.location - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

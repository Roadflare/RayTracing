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
        let up = Vector { x: 0.0, y: 1.0, z: 0.0 };
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

                let pixel_dir = forward
                    + right * (u * 2.0 * aspect_ratio)
                    + true_up * (-v * 2.0);

                let ray = Ray::new(self.coords, pixel_dir);
                let color: Color;
                match scene.trace_ray(&ray) {
                    None => {
                        color = Color::RGB(150, 170, 150 + (y as f64 * 105. / height as f64) as u8)
                    }
                    Some((sphere, point)) => {
                        let n = sphere.normal(point) * 255.;
                        color = Color::RGB(n.x as u8, n.y as u8, (-n.z) as u8)
                    }
                }

                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}

pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    fn trace_ray(&self, ray: &Ray) -> Option<(&Sphere, Vector)> {
        let mut closest: (f64, &Sphere, Vector) = (-1., &self.spheres[0], ray.origin);
        for sphere in self.spheres.iter() {
            let d = sphere.hit_distance(ray);
            if d > 0. && (d < closest.0 || closest.0 < 0.) {
                closest = (d, &sphere, ray.origin + ray.direction * d);
            }
        }
        if closest.0 < 0. { None }
        else { Some((closest.1, closest.2))}
    }
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn hit_distance(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. { -1. }
        else { (-b - discriminant.powf(0.5)) / (2. * a) }
    }

    pub fn normal(&self, point: Vector) -> Vector {
        (point - self.center) / self.radius
    }
}

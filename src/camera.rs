use crate::scene::{Scene,ColorType,Collision};
use crate::vectors::Vector;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;


pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

pub struct Camera {
    // Global camera position and orientation
    pub coords: Vector,
    pub direction: Vector,
}

struct CameraBasis {
    // Relative orientation of the camera
    forward: Vector,
    right: Vector,
    up: Vector,
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

    pub fn relocate(self, keycode: Keycode) -> Self {
        let base = self.camera_basis();
        let mut movement = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        match keycode {
            Keycode::W => {
                let forward = Vector {
                    x: base.forward.x,
                    y: 0.0,
                    z: base.forward.z,
                }
                .normalized();
                movement = movement + forward;
            }
            Keycode::S => {
                let backward = Vector {
                    x: base.forward.x,
                    y: 0.0,
                    z: base.forward.z,
                }
                .normalized();
                movement = movement - backward;
            }
            Keycode::A => movement = movement - base.right,
            Keycode::D => movement = movement + base.right,
            Keycode::Space => movement = movement + Vector::make(0.0, 1.0, 0.0),
            Keycode::LShift => movement = movement - Vector::make(0.0, 1.0, 0.0),
            _ => {}
        }

        Camera {
            coords: self.coords + movement.normalized(),
            direction: self.direction,
        }
    }

    pub fn rotate(&self, angle_degrees: f64) -> Self {
        let angle_rad = angle_degrees.to_radians();
        let cos_theta = angle_rad.cos();
        let sin_theta = angle_rad.sin();

        let dir = self.direction;
        let new_direction = Vector {
            x: dir.x * cos_theta - dir.z * sin_theta,
            y: dir.y,
            z: dir.x * sin_theta + dir.z * cos_theta,
        }
        .normalized();

        Camera {
            coords: self.coords,
            direction: new_direction,
        }
    }
    
    fn camera_basis(&self) -> CameraBasis {
        let up = Vector::make(0.0, 1.0, 0.0);
        let forward = self.direction;
        let right = forward.cross(&up).normalized();
        let true_up = right.cross(&forward).normalized();
        CameraBasis {
            forward,
            right,
            up: true_up,
        }
    }

    fn generate_ray(
        &self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        aspect_ratio: f64,
        basis: &CameraBasis,
    ) -> Ray {
        let u = (x as f64 + 0.5) / width as f64 - 0.5;
        let v = (y as f64 + 0.5) / height as f64 - 0.5;

        let direction =
            basis.forward + basis.right * (u * 2.0 * aspect_ratio) + basis.up * (-v * 2.0);

        Ray::new(self.coords, direction)
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        scene: &Scene,
        width: u16,
        ratio: (u16, u16),
    ) {
        let cam_basis = self.camera_basis();
        let (x_ratio, y_ratio) = ratio;
        let height = (width as f64 * (y_ratio as f64 / x_ratio as f64)) as u16;
        let aspect_ratio = x_ratio as f64 / y_ratio as f64;

        for y in 0..height {
            for x in 0..width {
                let ray = self.generate_ray(x, y, width, height, aspect_ratio, &cam_basis);
                let color = trace_color(scene, &ray, y, height);
                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}


fn trace_color(scene: &Scene, ray: &Ray, y: u16, height: u16) -> Color {
    let (brightness, base_color) = match ray.trace(scene) {
        Some(Collision::Sphere(sphere, point)) => {
            let brightness = compute_lighting(scene, point, sphere.normal(point));
            let base_color = match sphere.material.color {
                ColorType::Solid(c) => c,
                ColorType::Function(f) => f(point),
            };
            (brightness, base_color)
        },
        Some(Collision::Triangle(triangle, point)) => {
            let brightness = compute_lighting(scene, point, triangle.normal);
            let base_color = match triangle.material.color {
                ColorType::Solid(c) => c,
                ColorType::Function(f) => f(point)
            };
            (brightness, base_color)
        },
        None => { (1., Color::RGB(150, 170, 150 + (y as f64 * 105. / height as f64) as u8)) }
    };

    let r = (base_color.r as f64 * brightness) as u8;
    let g = (base_color.g as f64 * brightness) as u8;
    let b = (base_color.b as f64 * brightness) as u8;

    Color::RGB(r, g, b)
}

fn compute_lighting(scene: &Scene, point: Vector, normal: Vector) -> f64 {
    let mut brightness = scene.ambient_light;

    for light in &scene.lights {
        let light_dir = (light.position - point).normalized();

        // Pomaknjena začetna točka, da se izognemo samo-senci
        let shadow_ray = Ray::new(point + normal * 0.001, light_dir);
        let in_shadow = &shadow_ray.trace(scene).is_some();

        if !in_shadow {
            let light_contrib = normal.dot(&light_dir).max(0.0) * light.intensity;
            brightness += light_contrib;
        }
    }

    brightness.clamp(0.0, 1.0)
}

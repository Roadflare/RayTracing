use crate::scene::{Collision, ColorType, Material, Plane, Scene, Sphere, Triangle};
use crate::vectors::Vector;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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
                let forward = Vector::make(base.forward.x, 0.0, base.forward.z).normalized();
                movement = movement + forward;
            }
            Keycode::S => {
                let backward = Vector::make(base.forward.x, 0.0, base.forward.z).normalized();
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

    pub fn unepic(&self) -> Self {
        Camera::new(Vector::make(-3.0, 0.0, 0.0), Vector::make(1.0, 0.0, 0.0))
    }
    pub fn epic(&self) -> Self {
        Camera::new(Vector::make(3.0, 0.0, 0.0), Vector::make(-1.0, 0.0, 0.0))
    }

    pub fn rotate(&self, angle_degrees: f64) -> Self {
        let angle_rad = angle_degrees.to_radians();
        let cos_theta = angle_rad.cos();
        let sin_theta = angle_rad.sin();

        let dir = self.direction;
        let new_direction = Vector::make(
            dir.x * cos_theta - dir.z * sin_theta,
            dir.y,
            dir.x * sin_theta + dir.z * cos_theta,
        )
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
        depth: u32,
    ) {
        let cam_basis = self.camera_basis();
        let (x_ratio, y_ratio) = ratio;
        let height = (width as f64 * (y_ratio as f64 / x_ratio as f64)) as u16;
        let aspect_ratio = x_ratio as f64 / y_ratio as f64;

        for y in 0..height {
            for x in 0..width {
                let ray = self.generate_ray(x, y, width, height, aspect_ratio, &cam_basis);
                let color = trace_color(scene, &ray, y, height, depth)
                    .unwrap_or_else(|| background_color(ray.direction));
                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}

fn trace_color(scene: &Scene, ray: &Ray, y: u16, height: u16, depth: u32) -> Option<Color> {
    if depth == 0 {
        return Some(Color::RGB(0, 0, 0)); // Max depth reached
    }

    match ray.trace(scene) {
        Some(Collision::Sphere(sphere, point)) => Some(handle_hit(
            point,
            sphere.normal(point),
            &sphere.material,
            scene,
            ray,
            y,
            height,
            depth,
        )),

        Some(Collision::Triangle(triangle, point)) => Some(handle_hit(
            point,
            triangle.normal,
            &triangle.material,
            scene,
            ray,
            y,
            height,
            depth,
        )),
        Some(Collision::Plane(triangle, point)) => Some(handle_hit(
            point,
            triangle.normal,
            &triangle.material,
            scene,
            ray,
            y,
            height,
            depth,
        )),
        None => None,
    }
}

fn handle_hit(
    point_of_colision: Vector,
    normal: Vector,
    material: &Material,
    scene: &Scene,
    ray: &Ray,
    y: u16,
    height: u16,
    depth: u32,
) -> Color {
    let brightness = compute_lighting(scene, point_of_colision, normal);

    let base_color = match &material.color {
        ColorType::Solid(c) => *c,
        ColorType::Function(f) => f(point_of_colision),
    };

    let reflectivity = material.reflectivity;

    if reflectivity > 0.0 {
        let reflected_dir = ray.direction.reflect(&normal).normalized();
        let reflected_ray = Ray::new(point_of_colision + normal * 0.001, reflected_dir);
        let reflected_color = trace_color(scene, &reflected_ray, y, height, depth - 1);
        if reflected_color.is_none() {
            return blend_colors(
                base_color,
                background_color(reflected_dir),
                brightness,
                reflectivity,
            );
        }

        blend_colors(
            base_color,
            reflected_color.unwrap_or_else(|| background_color(ray.direction)),
            brightness,
            reflectivity,
        )
    } else {
        scale_color(base_color, brightness)
    }
}

fn scale_color(color: Color, brightness: f64) -> Color {
    Color::RGB(
        (color.r as f64 * brightness).clamp(0.0, 255.0) as u8,
        (color.g as f64 * brightness).clamp(0.0, 255.0) as u8,
        (color.b as f64 * brightness).clamp(0.0, 255.0) as u8,
    )
}

fn blend_colors(base: Color, reflected: Color, brightness: f64, reflectivity: f64) -> Color {
    let inv_r = 1.0 - reflectivity;
    Color::RGB(
        ((base.r as f64 * brightness * inv_r + reflected.r as f64 * reflectivity).clamp(0.0, 255.0))
            as u8,
        ((base.g as f64 * brightness * inv_r + reflected.g as f64 * reflectivity).clamp(0.0, 255.0))
            as u8,
        ((base.b as f64 * brightness * inv_r + reflected.b as f64 * reflectivity).clamp(0.0, 255.0))
            as u8,
    )
}

fn compute_lighting(scene: &Scene, point_of_colision: Vector, normal_of_hit_object: Vector) -> f64 {
    let mut brightness = scene.ambient_light;

    for light in &scene.lights {
        let light_dir = (light.position - point_of_colision).normalized();

        // Pomaknjena začetna točka, da se izognemo samo-senci
        let shadow_ray = Ray::new(point_of_colision + normal_of_hit_object * 0.001, light_dir);
        let trace_res = &shadow_ray.trace(scene);

        let light_distance = (light.position - point_of_colision).length();
        let mut in_shadow = false;

        match trace_res {
            Some(Collision::Sphere(_, p))
            | Some(Collision::Triangle(_, p))
            | Some(Collision::Plane(_, p)) => {
                let distance = (*p - point_of_colision).length();
                if distance < light_distance {
                    in_shadow = true;
                }
            }
            _ => {}
        }

        if !in_shadow {
            let light_contrib = normal_of_hit_object.dot(&light_dir).max(0.0) * light.intensity;
            brightness += light_contrib;
        }
    }

    brightness.clamp(0.0, 1.0)
}

fn background_color(dir: Vector) -> Color {
    let t = ((dir.y + 1.0) * 0.5).clamp(0.0, 1.0);

    // Preliv med spodaj (temno modra) in zgoraj (svetla rožnata)
    let bottom = (50.0, 80.0, 160.0);
    let top = (255.0, 200.0, 180.0);

    let r = (1.0 - t) * bottom.0 + t * top.0;
    let g = 255;
    let b = (1.0 - t) * bottom.2 + t * top.2;

    Color::RGB(r as u8, g as u8, b as u8)
}

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }
    pub fn trace<'a>(&'a self, scene: &'a Scene) -> Option<Collision<'a>> {
        let mut closest: Option<(f64, Collision)> = None;

        // Spheres
        for sphere in &scene.spheres {
            if let Some(dist) = self.sphere_hit_detection(sphere) {
                if closest.as_ref().map_or(true, |(d, _)| dist < *d) {
                    let hit_point = self.origin + self.direction * dist;
                    closest = Some((dist, Collision::Sphere(sphere, hit_point)));
                }
            }
        }

        // Triangles
        for triangle in &scene.triangles {
            if let Some(dist) = self.triangle_hit_detection(triangle) {
                if closest.as_ref().map_or(true, |(d, _)| dist < *d) {
                    let hit_point = self.origin + self.direction * dist;
                    closest = Some((dist, Collision::Triangle(triangle, hit_point)));
                }
            }
        }

        // Planes
        for plane in &scene.planes {
            if let Some(hit_point) = self.plane_hit_detection(plane) {
                let dist = (hit_point - self.origin).length();
                if closest.as_ref().map_or(true, |(d, _)| dist < *d) {
                    closest = Some((dist, Collision::Plane(plane, hit_point)));
                }
            }
        }

        closest.map(|(_, collision)| collision)
    }

    pub fn plane_hit_detection(&self, plane: &Plane) -> Option<Vector> {
        let denom = plane.normal.dot(&self.direction);
        if denom.abs() < 1e-6 {
            return None; // Ray je vzporeden ravnini
        }

        let t = (plane.point - self.origin).dot(&plane.normal) / denom;
        if t > 0.0 {
            Some(self.origin + self.direction * t)
        } else {
            None // Ravnina je za kamero
        }
    }

    pub fn sphere_hit_detection(&self, sphere: &Sphere) -> Option<f64> {
        let oc = self.origin - sphere.center;
        /*
        a = ||ray_dir||^2
        b = 2 ray_dir * oc
        c = ||oc||^2 - r^2

        D = b^2 - 4ac
        */
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let dist = (-b - discriminant.sqrt()) / (2.0 * a);
            if dist > 0.0 { Some(dist) } else { None }
        }
    }

    pub fn triangle_hit_detection(&self, triangle: &Triangle) -> Option<f64> {
        // algorithm from: https://www.lighthouse3d.com/tutorials/maths/ray-triangle-intersection/

        let (v0, v1, v2) = triangle.vertices;
        let (e1, e2) = (v1 - v0, v2 - v0);
        let h = self.direction.cross(&e2);
        let a = e1.dot(&h);
        if -0.00001 < a && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = self.origin - v0;
        let u = f * s.dot(&h);
        if u > 1. || u < 0. {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * self.direction.dot(&q);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = f * e2.dot(&q);
        if t > 0.001 { Some(t) } else { None }
    }
}

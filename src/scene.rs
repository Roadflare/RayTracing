use crate::vectors::Vector;
use crate::camera::Ray;
use sdl2::pixels::Color;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

impl Scene {
    pub fn make(spheres: Vec<Sphere>, lights: Vec<Light>, ambient_light: f64) -> Scene {
        Scene {
        spheres: spheres,
        lights: lights,
        ambient_light: ambient_light,
    }
}

    pub fn trace_ray(&self, ray: &Ray) -> Option<(&Sphere, Vector)> {
        let mut closest: (f64, &Sphere, Vector) = (-f64::INFINITY, &self.spheres[0], ray.origin);
        for sphere in self.spheres.iter() {
            let d = sphere.hit_distance(ray);
            if d > 0. && (d < closest.0 || closest.0 < 0.) {
                closest = (d, &sphere, ray.origin + ray.direction * d);
            }
        }
        if closest.0 < 0. { return None }
        Some((closest.1, closest.2))
    }
}
pub enum ColorType {
    Solid(Color),
    Function(fn(Vector) -> Color),//skalarno polje za barvo
}

pub struct Material {
    pub color: ColorType,
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn hit_distance(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant < 0. { -f64::INFINITY }
        else { (-b - discriminant.powf(0.5)) / (2. * a) }
    }

    pub fn normal(&self, point: Vector) -> Vector {
        (point - self.center) / self.radius
    }
}

pub struct Light {
    pub position: Vector,
    pub intensity: f64, // od 0.0 do 1.0
}
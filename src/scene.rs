use crate::camera::Ray;
use crate::vectors::Vector;
use sdl2::pixels::Color;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

pub enum ColorType {
    Solid(Color),
    Function(fn(Vector) -> Color), //skalarno polje za barvo
}

pub struct Material {
    pub color: ColorType,
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

pub struct Triangle {
    pub vertices: (Vector, Vector, Vector),
    pub normal: Vector,
    pub material: Material,
}

pub struct Light {
    pub position: Vector,
    pub intensity: f64, // od 0.0 do 1.0
}

pub enum Collision<'a> {
    Sphere(&'a Sphere, Vector),
    Triangle(&'a Triangle, Vector)
}

impl Scene {
    pub fn make(spheres: Vec<Sphere>, triangles: Vec<Triangle>, lights: Vec<Light>, ambient_light: f64) -> Scene {
        Scene {
            spheres: spheres,
            triangles: triangles,
            lights: lights,
            ambient_light: ambient_light,
        }
    }
}

impl Ray {
    pub fn trace<'a>(&'a self, scene: &'a Scene) -> Option<Collision<'a>> {
        let mut closest: (f64, Collision) = (-f64::INFINITY, Collision::Sphere(&scene.spheres[0], self.origin));
        if scene.spheres.len() > 0 {
            for sphere in scene.spheres.iter() {
                let distance = self.hit_distance(sphere);
                if distance > 0. && (distance < closest.0 || closest.0 < 0.) {
                    closest = (distance, Collision::Sphere(&sphere, self.origin + self.direction * distance));
                }
            }
        }
        if scene.triangles.len() > 0 {
            for triangle in scene.triangles.iter() {
                let distance = self.hit_distance_triangle(triangle);
                if distance > 0. && (distance < closest.0 || closest.0 < 0.) {
                    closest = (distance, Collision::Triangle(&triangle, self.origin + self.direction * distance));
                }
            }
        }
        if closest.0 < 0. { return None; }
        Some(closest.1)
    }

    pub fn hit_distance(&self, sphere: &Sphere) -> f64 {
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
        if discriminant < 0. { return -1. }
        return (-b - discriminant.powf(0.5)) / (2. * a)
    }

    pub fn hit_distance_triangle(&self, triangle: &Triangle) -> f64 {
        // algorithm from: https://www.lighthouse3d.com/tutorials/maths/ray-triangle-intersection/

        let (v0, v1, v2) = triangle.vertices;
        let (e1, e2) = (v1-v0, v2-v0);
        let h = self.direction.cross(&e2);
        let a = h.dot(&e1);
        if -0.00001 < a && a < 0.00001 { return -1. }

        let f = 1. / a;
        let s = v0 - self.origin;
        let u = f * s.dot(&h);
        if u > 1. || u < 0. { return -1. }

        let q = e1.cross(&s);
        let v = f * self.direction.dot(&q);
        if v < 0. || u + v > 1. { return -1. }

        let t = e2.dot(&q);
        if t <= 0.00001 { return -1. }
        
        t
    }

    fn reflect(&self, hitpoint: Vector, normal: &Vector) -> Ray {
        let dir = self.direction - *normal * (self.direction.dot(normal)) * 2.;
        return Ray{ direction: dir, origin: hitpoint};
    }
}


impl Sphere {
    pub fn make(center: &Vector, radius: f64, material: Material) -> Self {
        Sphere{ center: *center, radius: radius, material: material }
    }
    pub fn hit_distance(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant < 0. { return -f64::INFINITY }
        (-b - discriminant.powf(0.5)) / (2. * a)
    }

    pub fn normal(&self, point: Vector) -> Vector {
        (point - self.center) / self.radius
    }
}


impl Triangle {
    pub fn make(a: Vector, b: Vector, c: Vector, material: Material) -> Self {
        let (v1, v2) = (b - a, c - a);
        let n = v1.cross(&v2).normalized();
        Triangle { vertices: (a, b, c), normal: n, material: material }
    }
}
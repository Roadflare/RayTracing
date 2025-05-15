use crate::vectors::Vector;
use sdl2::pixels::Color;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
    pub planes: Vec<Plane>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

impl Scene {
    pub fn make(
        spheres: Vec<Sphere>,
        triangles: Vec<Triangle>,
        planes: Vec<Plane>,
        lights: Vec<Light>,
        ambient_light: f64,
    ) -> Scene {
        Scene {
            spheres,
            triangles,
            planes,
            lights,
            ambient_light,
        }
    }
}
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn make(center: &Vector, radius: f64, material: Material) -> Self {
        Sphere {
            center: *center,
            radius: radius,
            material: material,
        }
    }

    pub fn normal(&self, point: Vector) -> Vector {
        (point - self.center) / self.radius
    }
}

pub struct Material {
    pub color: ColorType,
    pub reflectivity: f64,
}

pub enum ColorType {
    Solid(Color),
    Function(fn(Vector) -> Color), //skalarno polje za barvo
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
    Triangle(&'a Triangle, Vector),
    Plane(&'a Plane, Vector),
}

impl Triangle {
    pub fn make(a: Vector, b: Vector, c: Vector, material: Material) -> Self {
        let (v1, v2) = (b - a, c - a);
        let n = v1.cross(&v2).normalized();
        Triangle {
            vertices: (a, b, c),
            normal: n,
            material: material,
        }
    }
}

pub struct Plane {
    pub point: Vector,  // Točka na ravnini
    pub normal: Vector, // Normala (naj bo normirana)
    pub material: Material,
}

use crate::vectors::Vector;
use sdl2::pixels::Color;
use image::RgbImage;
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
    Function(Box<dyn Fn(Vector) -> Color + Send + Sync>), // Send in sync sta zaradi thread safety-ja ker uporabljamo lazylock 
}

pub struct Triangle {
    pub vertices: (Vector, Vector, Vector),
    pub normal: Vector,
    pub material: Material,
}

pub struct Light {
    pub position: Vector,
    pub intensity: f64, // between 0.0 and 1.0
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
    pub point: Vector,  
    pub normal: Vector, 
    pub material: Material,
}

pub struct Texture {
    image: RgbImage,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> Texture {
        let img = image::open(path).expect("Failed to load texture").to_rgb8();
        let (w, h) = img.dimensions();
        Texture {
            image: img,
            width: w,
            height: h,
        }
    }

    pub fn uv_pixel_from_texture(&self, u: f64, v: f64) -> Color {
        let x = (u.fract() * self.width as f64) as u32;
        let y = ((1.0 - v.fract()) * self.height as f64) as u32;
        let pixel = self
            .image
            .get_pixel(x.min(self.width - 1), y.min(self.height - 1));
        Color::RGB(pixel[0], pixel[1], pixel[2])
    }
    pub fn sphere_uv(center: Vector, radius: f64, point: Vector) -> (f64, f64) {
        let p = (point - center) / radius; 
        let u = 0.5 - p.z.atan2(p.x) / (2.0 * std::f64::consts::PI);
        let v = 0.5 + p.y.asin() / std::f64::consts::PI;
        (u, v)
    }
}
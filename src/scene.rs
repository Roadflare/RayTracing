use crate::camera::Ray;
use crate::vectors::Vector;
use image::RgbImage;
use sdl2::pixels::Color;
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

impl Scene {
    pub fn make(objects: Vec<Box<dyn Hittable>>, lights: Vec<Light>, ambient_light: f64) -> Scene {
        Scene {
            objects,
            lights,
            ambient_light,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<f64>;
    fn normal(&self, point: Vector) -> Vector;
    fn material(&self) -> &Material;
}

pub struct Material {
    pub color: ColorType,
    pub reflectivity: Option<f64>,
    pub transparency: Option<(f64, f64)>,
}

pub enum ColorType {
    Solid(Color),
    Function(Box<dyn Fn(Vector) -> Color + Send + Sync>), // Send and Sync are safety features as LazyLock is used
}

pub struct Light {
    pub position: Vector,
    pub intensity: f64, // between 0.0 and 1.0
}

pub struct Collision<'a> {
    pub object: &'a dyn Hittable,
    pub point: Vector,
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
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let root2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if root1 > 0.001 {
                Some(root1)
            } else if root2 > 0.001 {
                Some(root2)
            } else {
                None
            }
        }
    }

    fn normal(&self, point: Vector) -> Vector {
        (point - self.center) / self.radius
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Triangle {
    pub vertices: (Vector, Vector, Vector),
    pub normal: Vector,
    pub material: Material,
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
impl Hittable for Triangle {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        // Algorithm from: https://www.lighthouse3d.com/tutorials/maths/ray-triangle-intersection/

        let (v0, v1, v2) = self.vertices;
        let (e1, e2) = (v1 - v0, v2 - v0);
        let h = ray.direction.cross(&e2);
        let a = e1.dot(&h);
        if -0.00001 < a && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(&h);
        if u > 1. || u < 0. {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = f * e2.dot(&q);
        if t > 0.001 { Some(t) } else { None }
    }
    fn normal(&self, _: Vector) -> Vector {
        self.normal
    }
    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Plane {
    pub point: Vector,
    pub normal: Vector,
    pub material: Material,
}
impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 {
            return None; // Ray is parallel to the plane
        }

        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        if t > 0.0 {
            Some(ray.origin + ray.direction * t)
        } else {
            None // Plane is behind the camera
        }
        .map(|p| (p - ray.origin).length())
    }

    fn normal(&self, _: Vector) -> Vector {
        self.normal
    }
    fn material(&self) -> &Material {
        &self.material
    }
}

use crate::scene::{Collision, ColorType, Material, Scene};
use crate::vectors::Vector;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const UP: Vector = Vector {
    x: 0.,
    y: 1.,
    z: 0.,
};

pub struct Camera {
    // Global camera position and orientation
    pub coords: Vector,
    pub direction: Vector,
}

struct CameraBasis {
    // Relative orientation of the camera
    forward: Vector,
    right: Vector,
}

impl Camera {
    pub fn new(coords: Vector, direction: Vector) -> Self {
        Camera {
            coords,
            direction: direction.normalized(),
        }
    }
    /// Relocates the camera based on the key pressed.
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

    /// Resets the camera to its initial position and direction.
    pub fn reset_location(&self) -> Self {
        Camera::new(Vector::make(-3.0, 0.0, 0.0), Vector::make(1.0, 0.0, 0.0))
    }

    /// Resets the camera to its initial position and direction, but reversed.
    pub fn reset_location_reversed(&self) -> Self {
        Camera::new(Vector::make(3.0, 0.0, 0.0), Vector::make(-1.0, 0.0, 0.0))
    }

    /// Rotates the camera around the Y-axis by the specified angle in degrees.
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

    /// Returns the camera's basis vectors for forward and right directions.
    fn camera_basis(&self) -> CameraBasis {
        let forward = self.direction;
        let right = forward.cross(&UP).normalized();
        CameraBasis { forward, right }
    }

    /// Generates a ray from the camera's position through a pixel at (x, y) on the screen.
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

        let direction = basis.forward + basis.right * (u * 2.0 * aspect_ratio) + UP * (-v * 2.0);

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

        for (_y, row) in (0..height).enumerate() {
            println!("Rendering row {}/{}", row + 1, height);

            for x in 0..width {
                let ray = self.generate_ray(x, row as u16, width, height, aspect_ratio, &cam_basis);
                let color = trace_color(scene, &ray, row as u16, height, depth)
                    .unwrap_or_else(|| background_color(ray.direction));

                canvas.set_draw_color(color);
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, row as i32));
            }
        }
    }
}

fn trace_color(scene: &Scene, ray: &Ray, y: u16, height: u16, depth: u32) -> Option<Color> {
    if depth == 0 {
        return Some(Color::RGB(0, 0, 0)); // Max depth reached
    }

    ray.trace(scene).map(|collision| {
        let object = collision.object;
        let point = collision.point;

        handle_hit(
            point,
            object.normal(point),
            object.material(),
            scene,
            ray,
            y,
            height,
            depth,
        )
    })
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
    let mut reflected_color: Option<Color> = None;
    let mut transparency: Option<f64> = None;
    let mut refracted_color: Option<Color> = None;
    let mut reflectivity: Option<f64> = None;

    if let Some((transparency_value, refraction_index)) = material.transparency {
        transparency = Some(transparency_value);
        let mut refraction_index = refraction_index;
        if ray.direction.dot(&normal) > 0. {
            refraction_index = 1. / refraction_index;
        }
        let refraction_dir = ray.direction.refract(&normal, refraction_index);
        let refracted_ray = Ray {
            origin: point_of_colision + refraction_dir * 0.001,
            direction: refraction_dir,
        };
        refracted_color = trace_color(scene, &refracted_ray, y, height, depth - 1);

        if refracted_color.is_none() {
            refracted_color = Some(background_color(refraction_dir));
        }
    }

    if let Some(reflectivity_value) = material.reflectivity {
        reflectivity = Some(reflectivity_value);
        let reflected_dir = ray.direction.reflect(&normal).normalized();
        let reflected_ray = Ray::new(point_of_colision + normal * 0.001, reflected_dir);
        reflected_color = trace_color(scene, &reflected_ray, y, height, depth - 1);
        
        if reflected_color.is_none() {
            reflected_color = Some(background_color(reflected_dir));
        }
    }

    blend_colors(base_color, reflected_color, refracted_color, reflectivity.unwrap_or(0.), transparency.unwrap_or(0.), brightness)
}

/*
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

    let mut color = Color::RGB(0, 0, 0);
    let mut refracted = false;
    if let Some((transparency, refraction_index)) = material.transparency {
        let refraction_dir = ray.direction.refract(&normal, refraction_index);
        let refracted_ray = Ray {
            origin: point_of_colision + refraction_dir * 0.001,
            direction: refraction_dir,
        };
        let refracted_color = trace_color(scene, &refracted_ray, y, height, depth);
        if refracted_color.is_none() {
            color = blend_colors(
                base_color,
                background_color(refracted_ray.direction),
                brightness,
                transparency,
            );
        } else {
            color = blend_colors(
                base_color,
                refracted_color.unwrap_or_else(|| background_color(ray.direction)),
                brightness,
                transparency,
            );
            refracted = true;
        }
    }

    if let Some(reflectivity) = material.reflectivity {
        let reflected_dir = ray.direction.reflect(&normal).normalized();
        let reflected_ray = Ray::new(point_of_colision + normal * 0.001, reflected_dir);
        let reflected_color = trace_color(scene, &reflected_ray, y, height, depth - 1);
        let mut display_color = Color::RGB(0, 0, 0);
        if reflected_color.is_none() {
            display_color = blend_colors(
                base_color,
                background_color(reflected_dir),
                brightness,
                reflectivity,
            );
        } else {
            display_color = blend_colors(
                base_color,
                reflected_color.unwrap_or_else(|| background_color(ray.direction)),
                brightness,
                reflectivity,
            );
        }
        if refracted {
            Color::RGB(
                (color.r + display_color.r) / 2,
                (color.g + display_color.g) / 2,
                (color.b + display_color.b) / 2,
            )
        } else {
            display_color
        }
    } else {
        scale_color(base_color, brightness)
    }
}
*/

/// Applies the illumination factor to a color
fn scale_color(color: Color, brightness: f64) -> Color {
    Color::RGB(
        (color.r as f64 * brightness).clamp(0.0, 255.0) as u8,
        (color.g as f64 * brightness).clamp(0.0, 255.0) as u8,
        (color.b as f64 * brightness).clamp(0.0, 255.0) as u8,
    )
}

/// Blends the reflected and base colors of a point
fn blend_colors(base: Color, reflected: Option<Color>, refracted: Option<Color>, reflectivity: f64, transparency: f64, brightness: f64) -> Color {
    let base_color = scale_color(base, brightness);
    match (reflected, refracted) {
        (Some(reflected_color), Some(refracted_color)) => {
            let transparency = transparency / 2.;
            let reflectivity = reflectivity / 2.;
            let base_weight = 1. - transparency - reflectivity;
            return Color::RGB(
                (base_color.r as f64 * base_weight + reflected_color.r as f64 * reflectivity + refracted_color.r as f64 * transparency) as u8,
                (base_color.g as f64 * base_weight + reflected_color.g as f64 * reflectivity + refracted_color.g as f64 * transparency) as u8,
                (base_color.b as f64 * base_weight + reflected_color.b as f64 * reflectivity + refracted_color.b as f64 * transparency) as u8,
            );
        }
        (Some(reflected_color), None) => {
            let base_weight = 1. - reflectivity;
            return Color::RGB(
                (base_color.r as f64 * base_weight + reflected_color.r as f64 * reflectivity) as u8,
                (base_color.g as f64 * base_weight + reflected_color.g as f64 * reflectivity) as u8,
                (base_color.b as f64 * base_weight + reflected_color.b as f64 * reflectivity) as u8,
            );
        }
        (None, Some(refracted_color)) => {
            let base_weight = 1. - transparency;
            return Color::RGB(
                (base_color.r as f64 * base_weight + refracted_color.r as f64 * transparency) as u8,
                (base_color.g as f64 * base_weight + refracted_color.g as f64 * transparency) as u8,
                (base_color.b as f64 * base_weight + refracted_color.b as f64 * transparency) as u8,
            );
        }
        _ => return base_color,
    }
}

/// Returns the illumination of a point based on its normal, other objects and light sources in the scene
fn compute_lighting(scene: &Scene, hit_point: Vector, normal: Vector) -> f64 {
    let mut brightness = scene.ambient_light;

    for light in &scene.lights {
        let light_dir = (light.position - hit_point).normalized();
        let light_distance = (light.position - hit_point).length();

        // We slightly offset the start of the shadow ray to prevent self-shadowing
        let shadow_ray = Ray::new(hit_point + normal * 0.001, light_dir);

        let in_shadow = if let Some(collision) = shadow_ray.trace(scene) {
            let dist = (collision.point - hit_point).length();
            dist < light_distance
        } else {
            false
        };

        if !in_shadow {
            let contribution = normal.dot(&light_dir).max(0.0) * light.intensity;
            brightness += contribution;
        }
    }

    brightness.clamp(0.0, 1.0)
}


/// Returns a color representing the background based on the direction of the given vector.
fn background_color(dir: Vector) -> Color {
    let t = ((dir.y + 1.0) * 0.5).clamp(0.0, 1.0);

    let top = (0., 0.0, 128.0);
    let bottom = (135., 206., 235.);

    let r = (1.0 - t) * bottom.0 + t * top.0;
    let g = 155;
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

    /// Returns Some of the closest collision of the ray 'self' or None if no collision occurs.
    pub fn trace<'a>(&'a self, scene: &'a Scene) -> Option<Collision<'a>> {
        scene
            .objects
            .iter()
            .filter_map(|obj| {
                obj.hit(self).map(|dist| {
                    let point = self.origin + self.direction * dist;
                    (
                        dist,
                        Collision {
                            object: &**obj,
                            point,
                        },
                    )
                })
            }) // Returns an iter of collissions of 'self' with objects
            .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap())
            .map(|(_, col)| col)
    }
}

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

 impl Vector {
	pub fn make (x_in: f64, y_in: f64, z_in: f64) -> Self {
		Vector {
            x: x_in,
            y: y_in,
            z: z_in
        }
	}

    fn length (&self) -> f64 {
        return f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2));
    }

    fn norm (self: &mut Vector) -> Vector {
        let length = self.length();
        if length == 0.0 {
            panic!("Zero Length!");
        }
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn dot(&self, other: &Vector) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn cross(self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    //projecira tako da kamera vidi. pozicije kamere se še ne da spreminjati(potrebna implementacija)
    pub fn project(v: &Self, width: f64, height: f64, fov: f64) -> (i32, i32) {
        let scale = fov / (fov + v.z);
        let x_proj = v.x * scale + width / 2.0;
        let y_proj = v.y * scale + height / 2.0;
        (x_proj as i32, y_proj as i32)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
        x: self.x + other.x,
        y: self.y + other.y,
        z: self.z + other.z
        }
    }
}

impl Sub for Vector {
    type Output =  Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
           z: self.z * other
       }
   }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}
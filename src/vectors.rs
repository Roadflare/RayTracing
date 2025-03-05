struct Vector {
	x: f64,
	y: f64,
	z: f64
}

impl Vector {
	fn make (x_in: f64, y_in: f64, z_in: f64) -> Self {
		Vector {
            x: x_in,
            y: y_in,
            z: z_in
        }
	}

    fn add (&self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    fn sub (&self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
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
}
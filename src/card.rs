struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
}

impl Add<Vec3, Vec3> for Vec3 {
    fn add(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3, Vec3> for Vec3 {
    fn sub(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Scalar Multiplication
impl Mul<f64, Vec3> for Vec3 {
    fn mul(&self, rhs: &f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Rem<Vec3, f64> for Vec3 {
    fn rem(&self, rhs: &f64) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

use std::fmt;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(nx: f64, ny: f64, nz: f64) -> Vec3 {
        Vec3 {
            x: nx,
            y: ny,
            z: nz,
        }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl fmt::Show for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({},{},{})", self.x, self.y, self.z)
    }
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
            x: self.x * *rhs,
            y: self.y * *rhs,
            z: self.z * *rhs,
        }
    }
}

/// Dot product
impl Rem<Vec3, f64> for Vec3 {
    fn rem(&self, rhs: &Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

/// Cross product
impl BitXor<Vec3, Vec3> for Vec3 {
    fn bitxor(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

/// Normalize
impl Not<Vec3> for Vec3 {
    fn not(&self) -> Vec3 {
        *self * (1.0f64 / (*self % *self).sqrt())
    }
}

#[cfg(test)]
mod test {
    use std::rand;
    use std::rand::Rng;
    use vec3::Vec3;

    static NUM_RAND_TESTS: uint = 256;
    static FLOAT_ERR: f64 = 1e-16;

    #[test]
    fn test_vec3_new() {
        // simple tests
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert!(a.x == 1.0);
        assert!(a.y == 2.0);
        assert!(a.z == 3.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            let z = rng.gen::<f64>();
            let b = Vec3::new(x,y,z);
            assert!(x == b.x);
            assert!(y == b.y);
            assert!(z == b.z);
        }

    }

    #[test]
    fn test_vec3_origin() {
        let a = Vec3::origin();
        assert!(a.x == 0.0);
        assert!(a.y == 0.0);
        assert!(a.z == 0.0);
    }

    #[test]
    fn test_vec3_add() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert!(c.x == 5.0);
        assert!(c.y == 7.0);
        assert!(c.z == 9.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            let bx = rng.gen::<f64>();
            let by = rng.gen::<f64>();
            let bz = rng.gen::<f64>();
            let a = Vec3::new(ax, ay, az);
            let b = Vec3::new(bx, by, bz);
            let c = a + b;
            assert!(c.x == ax + bx);
            assert!(c.y == ay + by);
            assert!(c.z == az + bz);
        }
    }

    #[test]
    fn test_vec3_sub() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = b - a;
        assert!(c.x == 3.0);
        assert!(c.y == 3.0);
        assert!(c.z == 3.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            let bx = rng.gen::<f64>();
            let by = rng.gen::<f64>();
            let bz = rng.gen::<f64>();
            let a = Vec3::new(ax, ay, az);
            let b = Vec3::new(bx, by, bz);
            let c = a - b;
            assert!(c.x == ax - bx);
            assert!(c.y == ay - by);
            assert!(c.z == az - bz);
        }
    }

    #[test]
    fn test_vec3_mul() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 3.0;
        let c = a * b;
        assert!(c.x == 3.0);
        assert!(c.y == 6.0);
        assert!(c.z == 9.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            let a = Vec3::new(ax, ay, az);
            let b = rng.gen::<f64>();
            let c = a * b;
            assert!(c.x == ax * b);
            assert!(c.y == ay * b);
            assert!(c.z == az * b);
        }
    }

    #[test]
    fn test_vec3_dot() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a % b;
        assert!(c == 32.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            let bx = rng.gen::<f64>();
            let by = rng.gen::<f64>();
            let bz = rng.gen::<f64>();
            let a = Vec3::new(ax, ay, az);
            let b = Vec3::new(bx, by, bz);
            let c = a % b;
            assert!(c == (ax * bx) + (ay * by) + (az * bz));
        }
    }

    #[test]
    fn test_vec3_cross() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a ^ b;
        assert!(c.x == -3.0);
        assert!(c.y == 6.0);
        assert!(c.z == -3.0);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            let bx = rng.gen::<f64>();
            let by = rng.gen::<f64>();
            let bz = rng.gen::<f64>();
            let a = Vec3::new(ax, ay, az);
            let b = Vec3::new(bx, by, bz);
            let c = a ^ b;
            assert!(c.x == (ay * bz) - (az * by));
            assert!(c.y == (az * bx) - (ax * bz));
            assert!(c.z == (ax * by) - (ay * bx));
        }
    }

    #[test]
    fn test_vec3_norm() {
        // simple test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let c = !a;
        let mut mag: f64 = (1.0f64 + 4.0 + 9.0).sqrt();
        assert!(c.x == 1.0 / mag);
        assert!(c.y == 2.0 / mag);
        assert!(c.z == 3.0 / mag);

        // randomized tests
        let mut rng = rand::task_rng();
        for i in range(0u, NUM_RAND_TESTS) {
            let ax = rng.gen::<f64>();
            let ay = rng.gen::<f64>();
            let az = rng.gen::<f64>();
            mag = (ax * ax + ay * ay + az * az).sqrt();
            let a = Vec3::new(ax, ay, az);
            let c = !a;
            assert!(c.x <= ax / mag + FLOAT_ERR);
            assert!(c.x >= ax / mag - FLOAT_ERR);
            assert!(c.y <= ay / mag + FLOAT_ERR);
            assert!(c.y >= ay / mag - FLOAT_ERR);
            assert!(c.z <= az / mag + FLOAT_ERR);
            assert!(c.z >= az / mag - FLOAT_ERR);
        }
    }

}

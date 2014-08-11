use std::rand::Rng;
use std::rand;
use std::iter::range_step;
use std::iter;
use std::num::pow;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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

fn R() -> f64 {
    let mut rng = rand::task_rng();
    rng.gen::<f64>()
}

static G: &'static [i32] = &[247570,280596,280600,249748,18578,18577,231184,16,16];

fn S(o: Vec3, d: Vec3) -> Vec3 {
    // Search for an intersection ray Vs World
    let (m, t, n) = T(o, d);

    if(m == 0) {
        return Vec3::new(0.7, 0.6, 1.0) * pow(1.0-d.z,4)
    }
    Vec3::new(0.0,0.0,0.0)
}

fn T(o: Vec3, d: Vec3) -> (u32, f64, Vec3) {
    (0, 0.0, Vec3::origin())
}

fn main() {
    print!("P6 512 512 255 ");  // PPM Header

    let g = !Vec3::new(-6.0,-16.0,0.0);           // Camera direction
    let a = !(Vec3::new(0.0,0.0,0.0)^g)*0.002;    // Camera up vector
    let b = !(g^a)*0.002;                   // The right vector
    let c = (a + b)*-256.0+g;                 //

    for y in iter::range_step(511i,-1i,-1i) {          // for each px column
        for x in iter::range_step(511i,-1i,-1i) {      // for each px row
            // Reuse Vec3 to store RGB colors
            let mut p = Vec3::new(13.0, 13.0, 13.0); // default color is almost black

            // cast 64 rays per pixel
            for i in range_step(64i, 0i, -1i) {

                // delta applied to the origin of the view (for Depth of field)
                let t = a*(R()-0.5)*99.0+b*(R()-0.5)*99.0; 
                // Set the camera focal point and cast the ray
                // Accumulate the color returned in the p variable
                p = S(
                    Vec3::new(17.0,16.0,8.0)+t,         // Ray Origin
                    !(t*-1.0+(a*(R()+(x as f64))+b*((y as f64)+R())+c)*16.0)  // Ray direction with random deltas
                                                        // for stochastic sampling
                )*3.5+p;    // +p for color accumulation
            }
            print!("{}{}{}", p.x as i32, p.y as i32, p.z as i32);
        }
    }
}

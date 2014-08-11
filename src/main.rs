use std::rand::Rng;
use std::rand;
use std::iter::range_step;
use std::iter;
use std::num::{pow, Float};

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

// (S)ample the world and return the pixel color for a ray passing by point o (Origin) and d
// (Direction)
fn S(o: Vec3, d: Vec3) -> Vec3 {
    // Search for an intersection ray Vs World
    // m - status
    // t - intersect distance
    // n - 
    let (m, t, n) = T(o, d);

    if(m == 0) {
        // No sphere was found and the ray goes upward.  Generate a sky color.
        return Vec3::new(0.7, 0.6, 1.0) * pow(1.0-d.z,4)
    }

    // A sphere was maybe hit
    
    let mut h: Vec3 = o + d * t;                                  // h = intersection coordinate
    let l = !(Vec3::new(9.0+R(),9.0+R(),16.0)+h*-1.0);  // direction to the light (with random 
                                                        // delta for soft shadows
    let r = d+n * (n % d * -2.0);                             // the half-vector
    
    // Calculated the lambertian factor
    let mut b = l % n;

    // Calculate the illumination factor.  Trace a ray from intersection to the light source to
    // make sure it reaches.
    let (i, _, _) = T(h, l);
    if(b < 0.0 || i > 0) {
        b = 0.0;
    }

    // calculate illumination factor
    let p: f64 = pow(l % r * (if(b > 0.0) {1.0} else {0.0}), 99);

    if(m == 1) { 
        // no sphere was hit and ray is going down.  Generate floor color
        h = h * 0.2;
        return (if(((h.x).ceil()+(h.y).ceil()) as u64 & 1 == 1) {
                Vec3::new(3.0, 1.0, 1.0)
            } else {
                Vec3::new(3.0, 3.0, 3.0)
            }) * (b * 0.2 + 0.1);
    }

    // m == 2
    return Vec3::new(p,p,p) + S(h, r) * 0.5

}

// The intersection test of line [o,v]
// Return 2 if a hit was found (and also return distance t and bouncing ray n)
// Return 1 if no hit was found but ray goes downward
// Return 0 if no hit was found but ray goes upward
//
fn T(o: Vec3, d: Vec3) -> (u8, f64, Vec3) {
    let mut t: f64  = 1e9;
    let mut i: u8   = 0;
    let mut p: f64  = o.z / d.z;
    let mut n: Vec3 = Vec3::origin();
    let mut m: u8 = 0;

    if(0.01<p) {
        t = p;
        n = Vec3::new(0.0, 0.0, 1.0);
        m = 1;
    }

    for k in range_step(18u, -1, -1) {
        for j in range_step(8u, -1, -1) {
            if(G[j] & 1 << k != 0) { // for this line j, is there a sphere at column i?
                let p: Vec3 = o + Vec3::new(-k as f64, 0.0, (-j-4) as f64);
                let b: f64 = p % d;
                let c: f64 = p % p - 1.0;
                let q: f64 = b * b - c;

                if(q > 0.0) {
                    let s: f64 = -b - q.sqrt();
                    if(s < t && s > 0.01) {
                        t = s;
                        n = !(p + d * t);
                        m = 2;
                    }
                }
            }
        }
    }

    return (m, t, n);

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

            let xf = x as f64;
            let yf = y as f64;

            // cast 64 rays per pixel
            for i in range_step(64i, 0i, -1i) {

                // delta applied to the origin of the view (for Depth of field)
                let t = a*(R()-0.5)*99.0+b*(R()-0.5)*99.0; 
                // Set the camera focal point and cast the ray
                // Accumulate the color returned in the p variable
                p = S(
                    Vec3::new(17.0,16.0,8.0)+t,                 // Ray Origin
                    !(t*-1.0+(a*(R()+xf)+b*(yf+R())+c)*16.0)    // Ray direction with random deltas 
                                                                // for stochastic sampling
                )*3.5+p;    // +p for color accumulation
            }
            print!("{}{}{}", p.x as u8 as char, p.y as u8 as char, p.z as u8 as char);
        }
    }
}

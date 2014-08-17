use std::rand::Rng;
use std::rand;
use std::iter::range_step;
use std::num::{pow, Float};

use vec3::Vec3;

pub fn r() -> f64 {
    let mut rng = rand::task_rng();
    rng.gen::<f64>()
}

static G: &'static [u32] = &[247570,280596,280600,249748,18578,18577,231184,16,16];

// (S)ample the world and return the pixel color for a ray passing by point o (Origin) and d
// (Direction)
pub fn sample(o: Vec3, d: Vec3) -> Vec3 {
    // debug!("s() received input:\t(o={}, d={})", o, d);

    // search for an intersection ray vs World.
    let (m, t, n) = trace(o, d);
    // debug!("s() received trace:\t(m={}, t={}, n={})",m,t,n);

    if m == 0 {
        // No sphere was found and the ray goes upward.  Generate a sky color.
        return Vec3::new(0.7, 0.6, 1.0) * pow(1.0-d.z,4)
    }

    // A sphere was maybe hit
    
    let mut h: Vec3 = o + d * t;                        // h = intersection coordinate
    let l = !(Vec3::new(9.0+r(),9.0+r(),16.0)+h*-1.0);  // direction to the light (with random 
                                                        // delta for soft shadows
    let r = d+n * (n % d * -2.0);                       // the half-vector
    
    // Calculated the lambertian factor
    let mut b = l % n;

    // Calculate the illumination factor.  Trace a ray from intersection to the light source to
    // make sure it reaches.
    let (i, _, _) = trace(h, l);
    if b < 0.0 || i > 0 {
        b = 0.0;
    }

    // calculate illumination factor
    let p: f64 = pow(l % r * (if b > 0.0 {1.0} else {0.0}), 99);

    if m == 1 { 
        // no sphere was hit and ray is going down.  Generate floor color
        h = h * 0.2;
        return (if ((h.x).ceil()+(h.y).ceil()) as u64 & 1 == 1 {
                Vec3::new(3.0, 1.0, 1.0)
            } else {
                Vec3::new(3.0, 3.0, 3.0)
            }) * (b * 0.2 + 0.1)
    }

    // m == 2
    return Vec3::new(p,p,p) + sample(h, r) * 0.5

}

// The intersection test of line [o,v]
// Return 2 if a hit was found (and also return distance t and bouncing ray n)
// Return 1 if no hit was found but ray goes downward
// Return 0 if no hit was found but ray goes upward
//
fn trace(o: Vec3, d: Vec3) -> (u8, f64, Vec3) {
    let mut t: f64  = 1e9;
    let p: f64  = -o.z / d.z;
    let mut n: Vec3 = Vec3::origin();
    let mut m: u8 = 0;

    if 0.01 < p {
        t = p;
        n = Vec3::new(0.0, 0.0, 1.0);
        m = 1;
    }

    let mut spheres_found = 0u;

    for k in range_step(18u, -1, -1) {
        for j in range_step(8u, -1, -1) {
            if G[j] & (1 << k) != 0 { // for this line j, is there a sphere at column i?
                spheres_found += 1;
                // there is a sphere, but does the ray hit it?
                let p: Vec3 = o + Vec3::new(-(k as f64), 0.0, -(j as f64)-4.0);
                let b: f64 = p % d;
                let c: f64 = p % p - 1.0;
                let q: f64 = b * b - c;

                // does the ray hit the sphere
                if q > 0.0 {
                    // it does. compute the distance camera-sphere
                    let s: f64 = -b - q.sqrt();

                    if s < t && s > 0.01 {
                        // so far this is the minimum distance.  save it.  Also compute the
                        // bouncing ray vector into 'n'
                        t = s;
                        n = !(p + d * t);
                        m = 2;
                    }
                }
            }
        }
    }

    if spheres_found > 0 {
        // debug!("Spheres found: {}", spheres_found);
    }

    return (m, t, n);
}

#[cfg(test)]
mod test {

}

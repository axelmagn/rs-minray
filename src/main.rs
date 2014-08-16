#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use std::iter;
use std::iter::range_step;

use vec3::Vec3;
use trace::r;
use trace::sample;

mod vec3;
mod trace;

fn main() {
    print!("P6 512 512 255 ");  // PPM Header

    let g = !Vec3::new(-6.0,-16.0,0.0);           // Camera direction
    let a = !(Vec3::new(0.0,0.0,1.0)^g)*0.002;    // Camera up vector
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
                let t = a*(r()-0.5)*99.0+b*(r()-0.5)*99.0; 
                // Set the camera focal point and cast the ray
                // Accumulate the color returned in the p variable
                let o = Vec3::new(17.0,16.0,8.0)+t;                 // Ray Origin
                let d = !(t*-1.0+(a*(r()+xf)+b*(yf+r())+c)*16.0);   // ray direction w. random del    
                debug!("g = {}", g);
                debug!("a = {}", a);
                debug!("b = {}", b);
                debug!("xf = {}", xf);
                debug!("yf = {}", yf);
                debug!("t = {}", t);
                debug!("o = {}", o);
                debug!("d = {}", d);
                p = sample( o, d)*3.5+p;    // +p for color accumulation
                debug!("p = {}", p);
            }
            // print!("\n\t{}{}{}:\t", p.x, p.y, p.z); // DEBUG
            print!("{}{}{}", p.x as u8 as char, p.y as u8 as char, p.z as u8 as char);
        }
    }
}

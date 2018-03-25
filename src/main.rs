#![deny(missing_docs)]
//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.

mod ray;
mod vec;

use vec::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            col *= 255.99;
            let ir = col[0] as u32;
            let ig = col[1] as u32;
            let ib = col[2] as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

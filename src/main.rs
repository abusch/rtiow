#![deny(missing_docs)]
#![allow(non_snake_case)]
//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.

mod hitable;
mod ray;
mod sphere;
mod vec;

use std::f32;

use hitable::{HitRecord, Hitable};
use ray::Ray;
use sphere::Sphere;
use vec::{unit_vector, Vec3};

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut hit_record = HitRecord::default();
    if world.hit(r, 0.0, f32::INFINITY, &mut hit_record) {
        0.5 * Vec3::new(
            hit_record.normal.x() + 1.,
            hit_record.normal.y() + 1.,
            hit_record.normal.z() + 1.,
        )
    } else {
        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::default();

    let mut world = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)) as Box<Hitable>);
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)) as Box<Hitable>);
    let foo: &[Box<Hitable>] = &world[..];
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray::new(
                &origin,
                &(&lower_left_corner + u * &horizontal + v * &vertical),
            );
            let mut col = color(&ray, &foo);
            col *= 255.99;
            let ir = col[0] as u32;
            let ig = col[1] as u32;
            let ib = col[2] as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

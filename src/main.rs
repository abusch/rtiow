#![deny(missing_docs)]
#![allow(non_snake_case)]
//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.

extern crate rand;

mod camera;
mod hitable;
mod ray;
mod sphere;
mod vec;

use std::f32;

use rand::Rng;

use camera::Camera;
use hitable::{HitRecord, Hitable};
use ray::Ray;
use sphere::Sphere;
use vec::{unit_vector, Vec3};

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let target = &rec.p + &rec.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(&rec.p, &(&target - &rec.p)), world)
    } else {
        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p;
    loop {
        p = 2. * Vec3::new(rng.next_f32(), rng.next_f32(), rng.next_f32()) - Vec3::new(1., 1., 1.);
        if p.squared_length() < 1. {
            break;
        }
    }

    p
}

fn main() {
    // width
    let nx = 200;
    // height
    let ny = 100;
    // number of samples
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut rng = rand::thread_rng();
    let camera = Camera::new();
    let mut world = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)) as Box<Hitable>);
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)) as Box<Hitable>);
    let foo: &[Box<Hitable>] = &world[..];
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _s in 0..ns {
                let u = (i as f32 + rng.next_f32()) / nx as f32;
                let v = (j as f32 + rng.next_f32()) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &foo);
            }
            col /= ns as f32;
            col = Vec3::new(f32::sqrt(col.r()), f32::sqrt(col.g()), f32::sqrt(col.b()));
            col *= 255.99;
            let ir = col[0] as u32;
            let ig = col[1] as u32;
            let ib = col[2] as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

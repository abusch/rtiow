#![deny(missing_docs)]
#![allow(non_snake_case)]
//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.

mod ray;
mod vec;

use ray::Ray;
use vec::{unit_vector, Vec3};

fn color(r: &Ray) -> Vec3 {
    let center = Vec3::new(0., 0., -1.);
    let t = hit_sphere(&center, 0.5, r);
    if t > 0. {
        let N = unit_vector(&(&r.point_at_parameter(t) - &center));
        return 0.5 * Vec3::new(N.x() + 1., N.y() + 1., N.z() + 1.);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        -1.0
    } else {
        (-b - f32::sqrt(discriminant)) / (2. * a)
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray::new(
                &origin,
                &(&lower_left_corner + u * &horizontal + v * &vertical),
            );
            let mut col = color(&ray);
            col *= 255.99;
            let ir = col[0] as u32;
            let ig = col[1] as u32;
            let ib = col[2] as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

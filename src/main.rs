//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.
#![deny(missing_docs)]
#![allow(non_snake_case)]

extern crate rand;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec;

use std::f32;
use std::sync::Arc;

use rand::Rng;

use camera::Camera;
use hitable::{HitRecord, Hitable};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use vec::{unit_vector, Vec3};

fn color(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50 && rec.mat.is_some()
            && rec.mat
                .as_ref()
                .cloned()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            // if we hit a surface with a material, recurse along the scattered ray
            attenuation * color(&scattered, world, depth + 1)
        } else {
            // return black
            Vec3::new(0.0, 0.0, 0.0)
        }
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

fn random_scene() -> Vec<Box<Hitable>> {
    let mut rng = rand::thread_rng();
    let n = 500;
    let mut list = Vec::with_capacity(n);
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )) as Box<Hitable>);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.next_f32();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.next_f32(),
                0.2,
                b as f32 + 0.9 * rng.next_f32(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<Material> = if choose_mat < 0.8 {
                    // diffuse
                    Arc::new(Lambertian::new(Vec3::new(
                        rng.next_f32() * rng.next_f32(),
                        rng.next_f32() * rng.next_f32(),
                        rng.next_f32() * rng.next_f32(),
                    )))
                } else if choose_mat < 0.95 {
                    // metal
                    Arc::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rng.next_f32()),
                            0.5 * (1.0 + rng.next_f32()),
                            0.5 * (1.0 + rng.next_f32()),
                        ),
                        0.5 * rng.next_f32(),
                    ))
                } else {
                    // dielectric
                    Arc::new(Dielectric::new(1.5))
                };
                list.push(Box::new(Sphere::new(center, 0.2, mat)) as Box<Hitable>);
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)) as Arc<Material>,
    )) as Box<Hitable>);
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.3, 0.1))) as Arc<Material>,
    )) as Box<Hitable>);
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)) as Arc<Material>,
    )) as Box<Hitable>);

    list
}

fn main() {
    // width
    let nx = 1200;
    // height
    let ny = 800;
    // number of samples
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut rng = rand::thread_rng();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_focus = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        0.1,
        dist_focus,
    );
    // let mut world = Vec::new();
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(0., 0., -1.),
    //     0.5,
    //     Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))) as Arc<Material>,
    // )) as Box<Hitable>);
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(0., -100.5, -1.),
    //     100.,
    //     Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))) as Arc<Material>,
    // )) as Box<Hitable>);
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(1., 0., -1.),
    //     0.5,
    //     Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.3), 1.)) as Arc<Material>,
    // )) as Box<Hitable>);
    // world.push(Box::new(Sphere::new(
    //     Vec3::new(-1., 0., -1.),
    //     0.5,
    //     Arc::new(Dielectric::new(1.5)) as Arc<Material>,
    // )) as Box<Hitable>);
    let world: &[Box<Hitable>] = &random_scene()[..];
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _s in 0..ns {
                let u = (i as f32 + rng.next_f32()) / nx as f32;
                let v = (j as f32 + rng.next_f32()) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
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

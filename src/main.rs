//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.
#![deny(missing_docs)]
#![allow(non_snake_case)]

extern crate rand;
#[macro_use]
extern crate lazy_static;

mod aabb;
mod bvh;
mod camera;
mod hitable;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod vec;

use std::f32;
use std::sync::Arc;

use rand::Rng;

use bvh::BvhNode;
use camera::Camera;
use hitable::{HitRecord, Hitable};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::{MovingSphere, Sphere};
use texture::{CheckerTexture, ConstantTexture, NoiseTexture};
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

fn two_perlin_spheres() -> Vec<Arc<Hitable>> {
    let perltext = Arc::new(NoiseTexture::new(4.0));
    let mut list = Vec::new();

    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perltext.clone())),
    )) as Arc<Hitable>);
    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perltext.clone())),
    )) as Arc<Hitable>);

    list
}

fn random_scene() -> Vec<Arc<Hitable>> {
    let mut rng = rand::thread_rng();
    let n = 500;
    let mut list = Vec::with_capacity(n);
    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
            Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Arc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
        )))),
    )) as Arc<Hitable>);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.next_f32();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.next_f32(),
                0.2,
                b as f32 + 0.9 * rng.next_f32(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.push(Arc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rng.next_f32(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::constant(Vec3::new(
                            rng.next_f32() * rng.next_f32(),
                            rng.next_f32() * rng.next_f32(),
                            rng.next_f32() * rng.next_f32(),
                        ))),
                    )) as Arc<Hitable>)
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.next_f32()),
                                0.5 * (1.0 + rng.next_f32()),
                                0.5 * (1.0 + rng.next_f32()),
                            ),
                            0.5 * rng.next_f32(),
                        )),
                    )) as Arc<Hitable>);
                } else {
                    // dielectric
                    list.push(
                        Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))))
                            as Arc<Hitable>,
                    );
                }
            }
        }
    }

    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)) as Arc<Material>,
    )) as Arc<Hitable>);
    list.push(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::constant(Vec3::new(0.4, 0.3, 0.1))) as Arc<Material>,
    )) as Arc<Hitable>);
    list.push(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)) as Arc<Material>,
    )) as Arc<Hitable>);

    list
}

fn main() {
    // width
    let nx = 400;
    // height
    let ny = 200;
    // number of samples
    let ns = 10;

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
        0.0,
        1.0,
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
    // let mut world = random_scene();
    let mut world = two_perlin_spheres();
    let bvh = BvhNode::new(&mut world, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _s in 0..ns {
                let u = (i as f32 + rng.next_f32()) / nx as f32;
                let v = (j as f32 + rng.next_f32()) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &bvh, 0);
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

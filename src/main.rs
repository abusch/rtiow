//! This is my implementation of the raytracer described in "Ray Tracing In One Weekend" by Peter
//! Shirley.
#![deny(missing_docs)]
#![allow(non_snake_case)]

extern crate rand;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;

mod aabb;
mod bvh;
mod camera;
mod hitable;
mod material;
mod perlin;
mod ray;
mod rect;
mod sphere;
mod texture;
mod vec;

use std::f32;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use rand::Rng;

use bvh::BvhNode;
use camera::Camera;
use hitable::*;
use material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use ray::Ray;
use rect::*;
use sphere::{MovingSphere, Sphere};
use texture::*;
use vec::Vec3;

fn color(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        let emitted = rec.mat.as_ref().unwrap().emitted(rec.u, rec.v, &rec.p);
        if depth < 50 && rec.mat.is_some()
            && rec.mat
                .as_ref()
                .cloned()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            // if we hit a surface with a material, recurse along the scattered ray
            emitted + attenuation * color(&scattered, world, depth + 1)
        } else {
            emitted
        }
    } else {
        Vec3::default()
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

fn cornell_box() -> Vec<Arc<Hitable>> {
    let perltext = Arc::new(NoiseTexture::new(4.0));
    let mut list: Vec<Arc<Hitable>> = Vec::new();
    let red: Arc<Material> = Arc::new(Lambertian::constant(Vec3::new(0.65, 0.05, 0.05)));
    let green: Arc<Material> = Arc::new(Lambertian::constant(Vec3::new(0.12, 0.45, 0.15)));
    let white: Arc<Material> = Arc::new(Lambertian::constant(Vec3::new(0.73, 0.73, 0.73)));
    let light: Arc<Material> = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
        Vec3::new(15.0, 15.0, 15.0),
    ))));

    list.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )))));
    list.push(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    list.push(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    list.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    list.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    list.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    list
}
fn simple_light() -> Vec<Arc<Hitable>> {
    let perltext = Arc::new(NoiseTexture::new(4.0));
    let mut list: Vec<Arc<Hitable>> = Vec::new();

    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perltext.clone())),
    )));
    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perltext.clone())),
    )));
    list.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
            Vec3::new(4.0, 4.0, 4.0),
        )))),
    )));
    list.push(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
            Vec3::new(4.0, 4.0, 4.0),
        )))),
    )));

    list
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
    env_logger::init();

    // width
    let nx = 400;
    // height
    let ny = 200;
    // number of samples
    let ns = 1000;

    let file = File::create("out.ppm").unwrap();
    let mut out = BufWriter::new(file);
    writeln!(out, "P3\n{} {}\n255", nx, ny).unwrap();

    info!("Rendering {}x{} image...", nx, ny);
    let mut rng = rand::thread_rng();
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_focus = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        0.0,
        0.0,
        1.0,
        dist_focus,
    );
    let mut world = cornell_box();
    // let mut world = random_scene();
    let bvh = BvhNode::new(&mut world[..], 0.0, 0.0);
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
            writeln!(out, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}

use std::fmt::Debug;

use hitable::HitRecord;
use random_in_unit_sphere;
use ray::Ray;
use vec::{unit_vector, Vec3};

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

/// Lambertian (diffuse) material. It scatters light uniformly in every direction (independently of
/// the viewing direction).
#[derive(Debug, Clone, Default)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = &rec.p + &rec.normal + random_in_unit_sphere();
        scattered.clone_from(&Ray::new(&rec.p, &(&target - &rec.p)));
        attenuation.clone_from(&self.albedo);
        true
    }
}

/// Metal material
#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        let f = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            &rec.p,
            &(&reflected + &(self.fuzz * random_in_unit_sphere())),
        );
        *attenuation = self.albedo.clone();

        Vec3::dot(scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let ni_over_nt;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        if Vec3::dot(r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -&rec.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
        }
        let reflected = reflect(r_in.direction(), &rec.normal);
        let mut refracted = Vec3::default();
        if refract(
            r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            *scattered = Ray::new(&rec.p, &refracted);
            true
        } else {
            *scattered = Ray::new(&rec.p, &reflected);
            false
        }
    }
}

// Utility functions

/// Returns the reflected vector of the given vector `v` wrt. the given normal `n`
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2. * Vec3::dot(v, n) * n)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant);
        true
    } else {
        false
    }
}

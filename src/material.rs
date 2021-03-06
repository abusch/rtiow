use std::fmt::Debug;
use std::sync::Arc;

use rand::{self, Rng};

use crate::hitable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::{ConstantTexture, Texture};
use crate::vec::{dot, unit_vector, Vec3};

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

/// Lambertian (diffuse) material. It scatters light uniformly in every direction (independently of
/// the viewing direction).
#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn constant(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(ConstantTexture::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::with_time(&rec.p, &(target - rec.p), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
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
        *scattered = Ray::with_time(
            &rec.p,
            &(reflected + self.fuzz * random_in_unit_sphere()),
            r_in.time(),
        );
        *attenuation = self.albedo;

        dot(scattered.direction(), &rec.normal) > 0.0
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
        let cosine;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let dir_dot_n = dot(r_in.direction(), &rec.normal);
        if dir_dot_n > 0.0 {
            outward_normal = -&rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dir_dot_n / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dir_dot_n / r_in.direction().length();
        }

        let reflected = reflect(r_in.direction(), &rec.normal);
        let mut refracted = Vec3::default();
        let reflect_prob = if refract(
            r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        if rand::thread_rng().gen::<f32>() < reflect_prob {
            *scattered = Ray::with_time(&rec.p, &reflected, r_in.time());
        } else {
            *scattered = Ray::with_time(&rec.p, &refracted, r_in.time());
        }

        true
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

// Utility functions

/// Returns the reflected vector of the given vector `v` wrt. the given normal `n`
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2. * dot(v, n) * n)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant);
        true
    } else {
        false
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

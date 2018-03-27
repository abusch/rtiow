use std::fmt::Debug;

use hitable::HitRecord;
use random_in_unit_sphere;
use ray::Ray;
use vec::Vec3;

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

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

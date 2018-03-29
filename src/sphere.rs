use std::sync::Arc;

use hitable::{HitRecord, Hitable};
use material::Material;
use ray::Ray;
use vec::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - &self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = Some(Arc::clone(&self.material));
                return true;
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = Some(Arc::clone(&self.material));
                return true;
            }
        }

        false
    }
}

use hitable::{HitRecord, Hitable};
use ray::Ray;
use vec::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - &self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
        }

        false
    }
}

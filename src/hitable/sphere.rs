use std::f32;
use std::sync::Arc;

use crate::aabb::{surrounding_box, Aabb};
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
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
                let (u, v) = get_sphere_uv(&((&rec.p - &self.center) / self.radius));
                rec.u = u;
                rec.v = v;
                return true;
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = Some(Arc::clone(&self.material));
                let (u, v) = get_sphere_uv(&((&rec.p - &self.center) / self.radius));
                rec.u = u;
                rec.v = v;
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        *aabb = Aabb::new(
            &(self.center - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center + Vec3::new(self.radius, self.radius, self.radius)),
        );

        true
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, t: f32) -> Vec3 {
        self.center0 + (t - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let center = self.center(r.time());
        let oc = r.origin() - &center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / self.radius;
                rec.mat = Some(Arc::clone(&self.material));
                return true;
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / self.radius;
                rec.mat = Some(Arc::clone(&self.material));
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        let aabb0 = Aabb::new(
            &(self.center0 - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center0 + Vec3::new(self.radius, self.radius, self.radius)),
        );
        let aabb1 = Aabb::new(
            &(self.center1 - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center1 + Vec3::new(self.radius, self.radius, self.radius)),
        );

        *aabb = surrounding_box(&aabb0, &aabb1);

        true
    }
}

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = f32::atan2(p.z(), p.x());
    let theta = f32::asin(p.y());
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::FRAC_PI_2) * f32::consts::FRAC_1_PI;

    (u, v)
}

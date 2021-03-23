use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;

#[derive(Debug)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    mp: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<dyn Material>) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.mat = Some(self.mp.clone());
        rec.p = r.point_at_parameter(t);
        rec.normal = Vec3::new(0.0, 0.0, 1.0);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        *aabb = Aabb::new(
            &Vec3::new(self.x0, self.y0, self.k - 0.0001),
            &Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

#[derive(Debug)]
pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    mp: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hitable for XZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.mat = Some(self.mp.clone());
        rec.p = r.point_at_parameter(t);
        rec.normal = Vec3::new(0.0, 1.0, 0.0);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        *aabb = Aabb::new(
            &Vec3::new(self.x0, self.k - 0.0001, self.z0),
            &Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}

#[derive(Debug)]
pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    mp: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hitable for YZRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.mat = Some(self.mp.clone());
        rec.p = r.point_at_parameter(t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);

        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        *aabb = Aabb::new(
            &Vec3::new(self.k - 0.0001, self.y0, self.z0),
            &Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}

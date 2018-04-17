use std::sync::Arc;

use aabb::Aabb;
use hitable::{HitRecord, Hitable};
use material::Material;
use ray::Ray;
use Vec3;

#[derive(Debug)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    mp: Arc<Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<Material>) -> XYRect {
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

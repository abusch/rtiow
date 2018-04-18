use std::sync::Arc;

use aabb::Aabb;
use hitable::{FlipNormals, HitRecord, Hitable, XYRect, XZRect, YZRect};
use material::Material;
use ray::Ray;
use Vec3;

pub struct Boxx {
    pmin: Vec3,
    pmax: Vec3,
    list_ptr: Vec<Arc<Hitable>>,
}

impl Boxx {
    pub fn new(p0: Vec3, p1: Vec3, mat: Arc<Material>) -> Boxx {
        let mut list: Vec<Arc<Hitable>> = Vec::new();

        list.push(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        list.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )))));
        list.push(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        list.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )))));
        list.push(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        list.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )))));

        Boxx {
            pmin: p0,
            pmax: p1,
            list_ptr: list,
        }
    }
}

impl Hitable for Boxx {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let list = &self.list_ptr[..];
        list.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut Aabb) -> bool {
        *aabb = Aabb::new(&self.pmin, &self.pmax);
        true
    }
}

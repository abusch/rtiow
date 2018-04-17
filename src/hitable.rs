use std::sync::Arc;

use aabb::{surrounding_box, Aabb};
use material::Material;
use ray::Ray;
use vec::Vec3;

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Option<Arc<Material>>,
    pub u: f32,
    pub v: f32,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut Aabb) -> bool;
}

impl<'a> Hitable for &'a [Arc<Hitable>] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_hit = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in self.iter() {
            if hitable.hit(r, t_min, closest_so_far, &mut temp_hit) {
                hit_anything = true;
                closest_so_far = temp_hit.t;
                rec.clone_from(&temp_hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut Aabb) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut temp_box = Aabb::default();
        let first_true = self[0].bounding_box(t0, t1, &mut temp_box);
        if !first_true {
            return false;
        } else {
            *aabb = temp_box.clone();
        }
        for hitable in self.iter().skip(1) {
            if hitable.bounding_box(t0, t1, &mut temp_box) {
                *aabb = surrounding_box(aabb, &temp_box);
            } else {
                return false;
            }
        }

        true
    }
}

pub struct FlipNormals {
    ptr: Arc<Hitable>,
}

impl FlipNormals {
    pub fn new(ptr: Arc<Hitable>) -> FlipNormals {
        FlipNormals { ptr }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let res = self.ptr.hit(r, t_min, t_max, rec);
        rec.normal = -rec.normal;

        res
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut Aabb) -> bool {
        self.ptr.bounding_box(t0, t1, aabb)
    }
}

use aabb::Aabb;
use hitable::{HitRecord, Hitable};
use ray::Ray;

pub struct BvhNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bbox: Aabb,
}

impl BvhNode {}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(r, t_min, t_max) {
            let mut left_rec = HitRecord::default();
            let mut right_rec = HitRecord::default();
            let hit_left = self.left.hit(r, t_min, t_max, &mut left_rec);
            let hit_right = self.right.hit(r, t_min, t_max, &mut right_rec);
            if hit_left && hit_right {
                if left_rec.t < right_rec.t {
                    rec.clone_from(&left_rec);
                } else {
                    rec.clone_from(&right_rec);
                }

                true
            } else if hit_left {
                rec.clone_from(&left_rec);
                true
            } else if hit_right {
                rec.clone_from(&right_rec);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut Aabb) -> bool {
        aabb.clone_from(&self.bbox);
        true
    }
}

use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use crate::aabb::{surrounding_box, Aabb};
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(l: &mut [Arc<dyn Hitable>], time0: f32, time1: f32) -> BvhNode {
        let mut rng = rand::thread_rng();
        let axis = (rng.gen::<f32>() * 3.0) as usize;
        match axis {
            0 => l.sort_by(box_x_compare),
            1 => l.sort_by(box_y_compare),
            2 => l.sort_by(box_z_compare),
            _ => unreachable!(),
        }
        let (left, right) = if l.len() == 1 {
            (Arc::clone(&l[0]), Arc::clone(&l[0]))
        } else if l.len() == 2 {
            (Arc::clone(&l[0]), Arc::clone(&l[1]))
        } else {
            let len = l.len();
            let (left_list, right_list) = l.split_at_mut(len / 2);
            (
                Arc::new(BvhNode::new(left_list, time0, time1)) as Arc<dyn Hitable>,
                Arc::new(BvhNode::new(right_list, time0, time1)) as Arc<dyn Hitable>,
            )
        };

        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();

        if !left.bounding_box(0.0, 0.0, &mut box_left)
            || !right.bounding_box(0.0, 0.0, &mut box_right)
        {
            panic!("No bounding box in BvhNode constructor!");
        }
        let bbox = surrounding_box(&box_left, &box_right);

        BvhNode { left, right, bbox }
    }
}

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

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut Aabb) -> bool {
        aabb.clone_from(&self.bbox);
        true
    }
}

fn box_x_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    let mut box_left = Aabb::default();
    let mut box_right = Aabb::default();

    if !a.bounding_box(0.0, 0.0, &mut box_left) || !b.bounding_box(0.0, 0.0, &mut box_right) {
        panic!("No bounding box in BvhNode constructor!");
    }

    box_left
        .min
        .x()
        .partial_cmp(&box_right.min.x())
        .expect("Bounding boxes contained NaN!")
}

fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    let mut box_left = Aabb::default();
    let mut box_right = Aabb::default();

    if !a.bounding_box(0.0, 0.0, &mut box_left) || !b.bounding_box(0.0, 0.0, &mut box_right) {
        panic!("No bounding box in BvhNode constructor!");
    }

    box_left
        .min
        .y()
        .partial_cmp(&box_right.min.y())
        .expect("Bounding boxes contained NaN!")
}

fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    let mut box_left = Aabb::default();
    let mut box_right = Aabb::default();

    if !a.bounding_box(0.0, 0.0, &mut box_left) || !b.bounding_box(0.0, 0.0, &mut box_right) {
        panic!("No bounding box in BvhNode constructor!");
    }

    box_left
        .min
        .z()
        .partial_cmp(&box_right.min.z())
        .expect("Bounding boxes contained NaN!")
}

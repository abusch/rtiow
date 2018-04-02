use std::f32;

use ray::Ray;
use vec::Vec3;

#[derive(Debug, Clone, Default)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: &Vec3, max: &Vec3) -> Aabb {
        Aabb {
            min: *min,
            max: *max,
        }
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction().length();
            let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                ::std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 > tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Vec3::new(
        f32::min(box0.min.x(), box1.min.x()),
        f32::min(box0.min.y(), box1.min.y()),
        f32::min(box0.min.z(), box1.min.z()),
    );
    let big = Vec3::new(
        f32::max(box0.max.x(), box1.max.x()),
        f32::max(box0.max.y(), box1.max.y()),
        f32::max(box0.max.z(), box1.max.z()),
    );
    Aabb::new(&small, &big)
}

use ray::Ray;
use vec::Vec3;

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

impl<'a> Hitable for &'a [Box<Hitable>] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_hit = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in self.iter() {
            if hitable.hit(r, t_min, closest_so_far, &mut temp_hit) {
                hit_anything = true;
                closest_so_far = temp_hit.t;
                rec.t = temp_hit.t;
                rec.p = temp_hit.p.clone();
                rec.normal = temp_hit.normal.clone();
            }
        }
        hit_anything
    }
}

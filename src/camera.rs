use std::f32;

use rand::{self, Rng};

use ray::Ray;
use vec::{cross, dot, unit_vector, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        // let lower_left_corner = Vec3::new(-half_width, -half_height, -1.);
        let lower_left_corner = origin - half_width * focus_distance * u
            - half_height * focus_distance * v - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();
        Ray::new(
            &(self.origin + offset),
            &(&self.lower_left_corner + s * &self.horizontal + t * &self.vertical - self.origin
                - offset),
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.next_f32(), rng.next_f32(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

use std::f32;
use ray::Ray;
use vec::{cross, unit_vector, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, fov: f32, aspect: f32) -> Camera {
        let theta = fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        // let lower_left_corner = Vec3::new(-half_width, -half_height, -1.);
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(&self.lower_left_corner + u * &self.horizontal + v * &self.vertical - self.origin),
        )
    }
}

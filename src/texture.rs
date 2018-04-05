use std::f32;
use std::fmt::Debug;
use std::sync::Arc;

use perlin;
use vec::Vec3;

pub trait Texture: Debug {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Debug, Clone, Default)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(c: Vec3) -> ConstantTexture {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    odd: Arc<Texture>,
    even: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Arc<Texture>, t1: Arc<Texture>) -> CheckerTexture {
        CheckerTexture { odd: t0, even: t1 }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = f32::sin(10.0 * p.x()) * f32::sin(10.0 * p.y()) * f32::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture { scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        // 0.5 * (1.0 + perlin::turb(&(self.scale * p), 7)) * Vec3::new(1.0, 1.0, 1.0)
        // perlin::turb(&(self.scale * p), 7) * Vec3::new(1.0, 1.0, 1.0)
        0.5 * (1.0 + f32::sin(self.scale * p.z() + 10.0 * perlin::turb(&(self.scale * p), 7)))
            * Vec3::new(1.0, 1.0, 1.0)
    }
}

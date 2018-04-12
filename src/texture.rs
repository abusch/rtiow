use std::f32;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use image::{self, GenericImage};

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

#[derive(Debug)]
pub struct ImageTexture {
    pub nx: u32,
    pub ny: u32,
    pub data: Box<[u8]>,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(filename: P) -> ImageTexture {
        let filename = filename.as_ref();
        let (img, nx, ny) = match image::open(filename) {
            Ok(data) => {
                let (nx, ny) = data.dimensions();
                (data.to_rgb().into_raw().into_boxed_slice(), nx, ny)
            }
            Err(e) => {
                warn!("Failed to open image {}: {}", filename.display(), e);
                (vec![128, 128, 128].into_boxed_slice(), 1, 1)
            }
        };
        info!(
            "Loaded texture {} with size {}x{}",
            filename.display(),
            nx,
            ny
        );

        ImageTexture { data: img, nx, ny }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let mut ifloat = u * self.nx as f32;
        let mut jfloat = v * self.ny as f32;
        if ifloat < 0.0 {
            ifloat = 0.0
        }
        if jfloat < 0.0 {
            jfloat = 0.0
        }
        if ifloat > (self.nx - 1) as f32 {
            ifloat = (self.nx - 1) as f32
        }
        if jfloat > (self.ny - 1) as f32 {
            jfloat = (self.ny - 1) as f32
        }
        let i = ifloat as usize;
        let j = jfloat as usize;
        let r = self.data[3 * i + 3 * self.nx as usize * j] as f32 / 255.0;
        let g = self.data[3 * i + 3 * self.nx as usize * j + 1] as f32 / 255.0;
        let b = self.data[3 * i + 3 * self.nx as usize * j + 2] as f32 / 255.0;

        Vec3::new(r, g, b)
    }
}

use std::f32;

use rand::{self, Rng};

use vec::{self, Vec3};

lazy_static! {
    static ref RANVEC: [Vec3; 256] = generate();
    static ref PERM_X: [usize; 256] = generate_perm();
    static ref PERM_Y: [usize; 256] = generate_perm();
    static ref PERM_Z: [usize; 256] = generate_perm();
}

pub fn turb(p: &Vec3, depth: u32) -> f32 {
    let mut accum = 0.0;
    let mut temp_p = p.clone();
    let mut weight = 1.0;

    for _ in 0..depth {
        accum += weight * noise(&temp_p);
        weight *= 0.5;
        temp_p *= 2.0;
    }
    accum.abs()
}

pub fn noise(p: &Vec3) -> f32 {
    let u = p.x() - f32::floor(p.x());
    let v = p.y() - f32::floor(p.y());
    let w = p.z() - f32::floor(p.z());
    let i = f32::floor(p.x()) as usize;
    let j = f32::floor(p.y()) as usize;
    let k = f32::floor(p.z()) as usize;
    let mut c = [[[Vec3::default(); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] =
                    RANVEC[PERM_X[(i + di) & 255] ^ PERM_Y[(j + dj) & 255] ^ PERM_Z[(k + dk) & 255]]
            }
        }
    }
    perlin_interp(&c, u, v, w)
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * vec::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}

fn permute(p: &mut [usize], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (1..n).rev() {
        let target = (rng.next_f32() * (i + 1) as f32) as usize;
        p.swap(i, target);
    }
}

fn generate() -> [Vec3; 256] {
    let mut rng = rand::thread_rng();

    let mut p = [Vec3::default(); 256];
    for i in 0..256 {
        p[i] = vec::unit_vector(&Vec3::new(
            -1.0 + 2.0 * rng.next_f32(),
            -1.0 + 2.0 * rng.next_f32(),
            -1.0 + 2.0 * rng.next_f32(),
        ));
    }
    p
}

fn generate_perm() -> [usize; 256] {
    let mut p = [0; 256];
    for i in 0..256 {
        p[i] = i;
    }
    permute(&mut p[..], 256);
    p
}

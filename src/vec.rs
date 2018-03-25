//! This crate implements a simple vector of 3 f32 components. It can be used for a position,
//! direction, colour, etc...
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        let k = self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], self.e[1], self.e[2])
    }
}

impl<'a> Add<f32> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, v: f32) -> Vec3 {
        Vec3::new(self.e[0] + v, self.e[1] + v, self.e[2] + v)
    }
}

impl<'a> Sub<f32> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, v: f32) -> Vec3 {
        Vec3::new(self.e[0] - v, self.e[1] - v, self.e[2] - v)
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Vec3 {
        Vec3::new(self.e[0] * v, self.e[1] * v, self.e[2] * v)
    }
}

impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Vec3 {
        debug_assert!(v != 0.0);
        let k = 1.0 / v;
        Vec3::new(self.e[0] * k, self.e[1] * k, self.e[2] * k)
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, v: f32) {
        self.e[0] += v;
        self.e[1] += v;
        self.e[2] += v;
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, v: f32) {
        self.e[0] -= v;
        self.e[1] -= v;
        self.e[2] -= v;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, v: f32) {
        self.e[0] *= v;
        self.e[1] *= v;
        self.e[2] *= v;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, v: f32) {
        debug_assert!(v != 0.0);
        let k = 1.0 / v;
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}
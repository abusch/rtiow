//! This crate implements a simple vector of 3 f32 components. It can be used for a position,
//! direction, colour, etc...
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
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

#[inline]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

#[inline]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        -(v1.x() * v2.z() - v1.z() * v2.x()),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
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

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - v.x(), self.e[1] - v.y(), self.e[2] - v.z())
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, v: &'b Vec3) -> Vec3 {
        Vec3::new(self.e[0] - v.x(), self.e[1] - v.y(), self.e[2] - v.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * v.x(), self.e[1] * v.y(), self.e[2] * v.z())
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Vec3 {
        Vec3::new(self.e[0] * v, self.e[1] * v, self.e[2] * v)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Vec3 {
        debug_assert!(v != 0.0);
        let k = 1.0 / v;
        Vec3::new(self.e[0] * k, self.e[1] * k, self.e[2] * k)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]
impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Vec3 {
        debug_assert!(v != 0.0);
        let k = 1.0 / v;
        Vec3::new(self.e[0] * k, self.e[1] * k, self.e[2] * k)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.e[0] += v.x();
        self.e[1] += v.y();
        self.e[2] += v.z();
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

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl<'a, 'b> Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, v: &'a Vec3) -> Vec3 {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl<'a> Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: &'a Vec3) -> Vec3 {
        Vec3::new(v.e[0] * self, v.e[1] * self, v.e[2] * self)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.e[0] * self, v.e[1] * self, v.e[2] * self)
    }
}

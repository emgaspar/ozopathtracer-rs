use core::fmt;
use core::fmt::Display;

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, DivAssign};

use crate::random::{random_f64, random_f64_range};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new (x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub const fn zeros() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub const fn ones() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn random() -> Vec3 {
        Vec3 { x: random_f64(), y: random_f64(), z: random_f64() }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 { 
            x: random_f64_range(min, max),
            y: random_f64_range(min, max),
            z: random_f64_range(min, max)
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::zeros();
        while p.length() < 1.0 {
            p = Vec3::random_range(-1.0, 1.0);
        }
        p
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();

        if dot(on_unit_sphere, *normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt()
        }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn clamp(self, min: f64, max: f64) -> Vec3 {
        let x: f64 = if self.x >= min { self.x } else { min };
        let y: f64 = if self.y >= min { self.y } else { min };
        let z: f64 = if self.z >= min { self.z } else { min };

        Vec3 {
            x: if x <= max { x } else { max },
            y: if y <= max { y } else { max },
            z: if z <= max { z } else { max },
        }

    }

    pub fn near_zero(self) -> bool {
        const THRESHOLD: f64 = 1.0e-8;
        (self.x < THRESHOLD) && (self.y < THRESHOLD) && (self.z < THRESHOLD)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs.dot(rhs)
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * (n)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = f64::min(dot(-uv, n), 1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: Vec3 = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
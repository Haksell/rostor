use std::ops::{Add, BitXor, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    e1: f64,
    e2: f64,
    e3: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e1, e2, e3 }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
    }

    pub fn inverse(self) -> Self {
        debug_assert!(!self.is_zero());
        self / self.length_squared()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn is_close(self, rhs: Self) -> bool {
        (self - rhs).length_squared() < 1e-10
    }

    pub fn is_zero(self) -> bool {
        self.e1 == 0.0 && self.e2 == 0.0 && self.e3 == 0.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BiVec3 {
    e12: f64,
    e23: f64,
    e31: f64,
}

impl BiVec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(e12: f64, e23: f64, e31: f64) -> Self {
        Self { e12, e23, e31 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TriVec3 {
    e123: f64,
}

impl TriVec3 {
    pub const ZERO: Self = Self::new(0.0);

    pub const fn new(e123: f64) -> Self {
        Self { e123 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MultiVec3 {
    e: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e12: f64,
    e23: f64,
    e31: f64,
    e123: f64,
}

impl MultiVec3 {
    pub const ZERO: Self = Self::new(0.0, Vec3::ZERO, BiVec3::ZERO, TriVec3::ZERO);

    pub const fn new(scalar: f64, vec3: Vec3, bivec3: BiVec3, trivec3: TriVec3) -> Self {
        Self {
            e: scalar,
            e1: vec3.e1,
            e2: vec3.e2,
            e3: vec3.e3,
            e12: bivec3.e12,
            e23: bivec3.e23,
            e31: bivec3.e31,
            e123: trivec3.e123,
        }
    }
}

impl BitXor for Vec3 {
    type Output = BiVec3;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BiVec3::new(
            self.e1 * rhs.e2 - self.e2 * rhs.e1,
            self.e2 * rhs.e3 - self.e3 * rhs.e2,
            self.e3 * rhs.e1 - self.e1 * rhs.e3,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e1 * rhs, self.e2 * rhs, self.e3 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e1 / rhs, self.e2 / rhs, self.e3 / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.e1 + rhs.e1, self.e2 + rhs.e2, self.e3 + rhs.e3)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.e1 - rhs.e1, self.e2 - rhs.e2, self.e3 - rhs.e3)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = MultiVec3;

    fn mul(self, rhs: Self) -> Self::Output {
        MultiVec3::new(self.dot(rhs), Vec3::ZERO, self ^ rhs, TriVec3::ZERO)
    }
}

impl Mul<MultiVec3> for MultiVec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            e: self.e * rhs.e + self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
                - self.e12 * rhs.e12
                - self.e23 * rhs.e23
                - self.e31 * rhs.e31
                - self.e123 * rhs.e123,

            e1: self.e * rhs.e1 + self.e1 * rhs.e - self.e2 * rhs.e12
                + self.e3 * rhs.e31
                + self.e12 * rhs.e2
                - self.e31 * rhs.e3
                - self.e23 * rhs.e123
                - self.e123 * rhs.e23,

            e2: self.e * rhs.e2 + self.e2 * rhs.e + self.e1 * rhs.e12
                - self.e3 * rhs.e23
                - self.e12 * rhs.e1
                + self.e23 * rhs.e3
                - self.e31 * rhs.e123
                - self.e123 * rhs.e31,

            e3: self.e * rhs.e3 + self.e3 * rhs.e - self.e1 * rhs.e31 + self.e2 * rhs.e23
                - self.e12 * rhs.e123
                - self.e23 * rhs.e2
                + self.e31 * rhs.e1
                - self.e123 * rhs.e12,

            e12: self.e * rhs.e12 + self.e12 * rhs.e + self.e1 * rhs.e2 - self.e2 * rhs.e1
                + self.e3 * rhs.e123
                - self.e23 * rhs.e31
                + self.e31 * rhs.e23
                + self.e123 * rhs.e3,

            e23: self.e * rhs.e23 + self.e23 * rhs.e + self.e1 * rhs.e123 + self.e2 * rhs.e3
                - self.e3 * rhs.e2
                + self.e12 * rhs.e31
                - self.e31 * rhs.e12
                + self.e123 * rhs.e1,

            e31: self.e * rhs.e31 + self.e31 * rhs.e - self.e1 * rhs.e3
                + self.e2 * rhs.e123
                + self.e3 * rhs.e1
                - self.e12 * rhs.e23
                + self.e23 * rhs.e12
                + self.e123 * rhs.e2,

            e123: self.e * rhs.e123
                + self.e123 * rhs.e
                + self.e1 * rhs.e23
                + self.e23 * rhs.e1
                + self.e2 * rhs.e31
                + self.e31 * rhs.e2
                + self.e3 * rhs.e12
                + self.e12 * rhs.e3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wedge_product() {
        assert_eq!(
            Vec3::new(1.0, 0.0, 0.0) ^ Vec3::new(0.0, -2.0, 0.0),
            BiVec3::new(-2.0, 0.0, 0.0)
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) ^ Vec3::new(1.0, 2.0, 3.0),
            BiVec3::ZERO
        );
    }

    #[test]
    fn inverse() {
        assert!(
            Vec3::new(6.0, 8.0, 0.0)
                .inverse()
                .is_close(Vec3::new(0.06, 0.08, 0.0))
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) ^ Vec3::new(1.0, 2.0, 3.0),
            BiVec3::ZERO
        );
    }
}

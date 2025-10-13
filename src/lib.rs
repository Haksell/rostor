use std::ops::BitXor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e1: f64,
    e2: f64,
    e3: f64,
}

impl Vec3 {
    pub const fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e1, e2, e3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiVec3 {
    e12: f64,
    e23: f64,
    e31: f64,
}

impl BiVec3 {
    pub const ZERO: BiVec3 = BiVec3::new(0.0, 0.0, 0.0);

    pub const fn new(e12: f64, e23: f64, e31: f64) -> Self {
        Self { e12, e23, e31 }
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
}

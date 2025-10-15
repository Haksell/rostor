use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

const EPS: f64 = 1e-7;

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
        (self.e1 - rhs.e1).abs() < EPS
            && (self.e2 - rhs.e2).abs() < EPS
            && (self.e3 - rhs.e3).abs() < EPS
    }

    pub fn is_zero(self) -> bool {
        self.e1 == 0.0 && self.e2 == 0.0 && self.e3 == 0.0
    }

    pub fn reflected_by(self, axis: Self) -> Self {
        // Derived from ava⁻¹ (self * axis * self.inverse())
        // https://jacquesheunis.com/post/rotors/#reflections-with-the-geometric-product
        let (a1, a2, a3) = axis.into();
        let (v1, v2, v3) = self.into();
        let p1 = a1 * a1 * v1 - a2 * a2 * v1 - a3 * a3 * v1 + 2. * a1 * a2 * v2 + 2. * a3 * a1 * v3;
        let p2 = a2 * a2 * v2 - a3 * a3 * v2 - a1 * a1 * v2 + 2. * a2 * a3 * v3 + 2. * a1 * a2 * v1;
        let p3 = a3 * a3 * v3 - a1 * a1 * v3 - a2 * a2 * v3 + 2. * a3 * a1 * v1 + 2. * a2 * a3 * v2;
        Self::new(p1, p2, p3) / axis.length_squared()
    }

    fn normalized(self) -> Self {
        self / self.length()
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(v: Vec3) -> Self {
        (v.e1, v.e2, v.e3)
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
pub struct Rotor3 {
    e: f64,
    e12: f64,
    e23: f64,
    e31: f64,
}

impl Rotor3 {
    pub const ZERO: Self = Self::new(0.0, BiVec3::ZERO);

    // Any rotor with only a scalar component is an identity
    pub const IDENTITY: Self = Self::new(1.0, BiVec3::ZERO);

    pub const fn new(scalar: f64, bivec3: BiVec3) -> Self {
        Self {
            e: scalar,
            e12: bivec3.e12,
            e23: bivec3.e23,
            e31: bivec3.e31,
        }
    }

    // Doesn't work if from from ≈ to
    pub fn from_to(from: Vec3, to: Vec3) -> Self {
        let from = from.normalized();
        let to = to.normalized();
        let halfway = (from + to).normalized();
        from * halfway
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DualRotor3 {
    e1: f64,
    e2: f64,
    e3: f64,
    e123: f64,
}

impl DualRotor3 {
    pub const ZERO: Self = Self::new(Vec3::ZERO, TriVec3::ZERO);

    pub const fn new(vec3: Vec3, trivec3: TriVec3) -> Self {
        Self {
            e1: vec3.e1,
            e2: vec3.e2,
            e3: vec3.e3,
            e123: trivec3.e123,
        }
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

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.e1, -self.e2, -self.e3)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.e1 * rhs, self.e2 * rhs, self.e3 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.e1 / rhs, self.e2 / rhs, self.e3 / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.e1 + rhs.e1, self.e2 + rhs.e2, self.e3 + rhs.e3)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.e1 - rhs.e1, self.e2 - rhs.e2, self.e3 - rhs.e3)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Rotor3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.dot(rhs), self ^ rhs)
    }
}

impl Mul<Vec3> for Rotor3 {
    type Output = DualRotor3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e1: self.e * rhs.e1 + self.e12 * rhs.e2 - self.e31 * rhs.e3,
            e2: self.e * rhs.e2 + self.e23 * rhs.e3 - self.e12 * rhs.e1,
            e3: self.e * rhs.e3 + self.e31 * rhs.e1 - self.e23 * rhs.e2,
            e123: self.e12 * rhs.e3 + self.e23 * rhs.e1 + self.e31 * rhs.e2,
        }
    }
}

impl Mul<Rotor3> for Rotor3 {
    type Output = Rotor3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            e: self.e * rhs.e - self.e12 * rhs.e12 - self.e23 * rhs.e23 - self.e31 * rhs.e31,
            e12: self.e * rhs.e12 + self.e12 * rhs.e - self.e23 * rhs.e31 + self.e31 * rhs.e23,
            e23: self.e * rhs.e23 + self.e23 * rhs.e + self.e12 * rhs.e31 - self.e31 * rhs.e12,
            e31: self.e * rhs.e31 + self.e31 * rhs.e - self.e12 * rhs.e23 + self.e23 * rhs.e12,
        }
    }
}

impl Mul<Vec3> for DualRotor3 {
    type Output = Rotor3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3,
            e12: self.e1 * rhs.e2 - self.e2 * rhs.e1 + self.e123 * rhs.e3,
            e23: self.e2 * rhs.e3 - self.e3 * rhs.e2 + self.e123 * rhs.e1,
            e31: -self.e1 * rhs.e3 + self.e3 * rhs.e1 + self.e123 * rhs.e2,
        }
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

impl Mul<Vec3> for MultiVec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self {
            e: self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3,
            e1: self.e * rhs.e1 + self.e12 * rhs.e2 - self.e31 * rhs.e3,
            e2: self.e * rhs.e2 - self.e12 * rhs.e1 + self.e23 * rhs.e3,
            e3: self.e * rhs.e3 - self.e23 * rhs.e2 + self.e31 * rhs.e1,
            e12: self.e1 * rhs.e2 - self.e2 * rhs.e1 + self.e123 * rhs.e3,
            e23: self.e2 * rhs.e3 - self.e3 * rhs.e2 + self.e123 * rhs.e1,
            e31: -self.e1 * rhs.e3 + self.e3 * rhs.e1 + self.e123 * rhs.e2,
            e123: self.e23 * rhs.e1 + self.e31 * rhs.e2 + self.e12 * rhs.e3,
        }
    }
}

impl TryFrom<DualRotor3> for Vec3 {
    type Error = ();

    fn try_from(m: DualRotor3) -> Result<Self, Self::Error> {
        if m.e123.abs() < EPS {
            Ok(Vec3::new(m.e1, m.e2, m.e3))
        } else {
            Err(())
        }
    }
}

impl TryFrom<MultiVec3> for Vec3 {
    type Error = ();

    fn try_from(m: MultiVec3) -> Result<Self, Self::Error> {
        if m.e.abs() < EPS
            && m.e12.abs() < EPS
            && m.e23.abs() < EPS
            && m.e31.abs() < EPS
            && m.e123.abs() < EPS
        {
            Ok(Vec3::new(m.e1, m.e2, m.e3))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::f64::consts::TAU};

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

    #[test]
    fn reflect() {
        assert_eq!(
            Vec3::new(0.0, 2.0, 0.0).reflected_by(Vec3::new(5.0, 0.0, 0.0)),
            Vec3::new(0.0, -2.0, 0.0)
        );
        assert_eq!(
            Vec3::new(0.0, 2.0, 0.0).reflected_by(Vec3::new(1.0, 1.0, 0.0)),
            Vec3::new(2.0, 0.0, 0.0)
        );
    }

    #[test]
    fn rotation() {
        let v = Vec3::new(1., 0., 1.);

        let (sin_a, cos_a) = (TAU * 2. / 24.).sin_cos();
        let a = Vec3::new(cos_a, sin_a, 0.);

        let (sin_b, cos_b) = (TAU * 7. / 24.).sin_cos();
        let b = Vec3::new(cos_b, sin_b, 0.);

        let (sin_c, cos_c) = (TAU * 10. / 24.).sin_cos();
        let c = Vec3::new(cos_c, sin_c, 1.);

        let rot = b * a;

        dbg!(rot * v);
        dbg!(rot * v * a.inverse());
        dbg!(rot * v * a.inverse() * b.inverse());
        dbg!(c);

        let res: Vec3 = TryFrom::try_from(rot * v * a.inverse() * b.inverse()).unwrap();

        assert!(res.is_close(c));
    }
}

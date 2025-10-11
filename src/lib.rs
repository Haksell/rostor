// TODO: impl Xor

#[derive(Debug, Clone, Copy)]
struct Infinity;

#[derive(Debug, Clone, Copy)]
struct Point;

#[derive(Debug, Clone, Copy)]
struct OrientedPointPair;

#[derive(Debug, Clone, Copy)]
struct OrientedCircle;

#[derive(Debug, Clone, Copy)]
struct OrientedLine;

#[derive(Debug, Clone, Copy)]
struct OrientedPlane;

impl Point {
    fn wedge_point(_: Point, _: &Point) -> OrientedPointPair {
        OrientedPointPair
    }
}

impl OrientedPointPair {
    fn wedge_point(&self, _: &Point) -> OrientedCircle {
        OrientedCircle
    }

    fn wedge_infinity(&self, _: &Infinity) -> OrientedLine {
        OrientedLine
    }
}

// impl OrientedLine {
//     fn dual(&self) -> Self {
//         Self
//     }
// }

impl OrientedCircle {
    fn wedge_infinity(&self, _: &Infinity) -> OrientedPlane {
        OrientedPlane
    }
}

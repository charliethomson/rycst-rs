
use coffee::graphics::{
    Mesh,
    Rectangle,
    Shape,
    Point,
    Color,
};

use crate::util::remap;

use ncollide2d::{
    math::Point as NPoint,
    shape::Polyline,
};

#[derive(Clone)]
pub struct Wall {
    internal: Polyline<f32>,
    pub color: Color,
} impl Wall {
    pub fn new(points: Vec<NPoint<f32>>, col: Color) -> Self {
        Self {
            internal: Polyline::new(points, None),
            color: col,
        }
    }

    pub fn map_into(&self, minmax: (i64, i64), rect: &Rectangle<f32>) -> Vec<Point> {

        self.points()
            .iter()
            .map(|point| (point.x, point.y))
            .map(|(x, y)| {
                let (mx, my) = minmax;
                Point::new(
                    remap(x, 0.0, mx as f32, rect.x, rect.width),
                    remap(y, 0.0, my as f32, rect.y, rect.height),
                )
            }).collect()

    }

    pub fn to_internal(&self) -> Polyline<f32> {
        Polyline::new(self.internal.points().to_vec(), None)
    }

    pub fn points(&self) -> Vec<Point> {
        self.internal.points().iter().map(|np| Point::new(np.x, np.y)).collect()
    }

    pub fn npoints(&self) -> Vec<NPoint<f32>> {
        self.internal.points().to_vec()
    }
}
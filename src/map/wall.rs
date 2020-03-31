
use coffee::graphics::{
    Mesh,
    Rectangle,
    Shape,
    Point,
    Color,
};

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

    pub fn draw_to_mesh(&self, mesh: &mut Mesh, window: &Rectangle<f32>) {
        mesh.stroke(Shape::Polyline {
                points: self.points().iter().map(|point| Point::new(point.x + window.x, point.y + window.y)).collect(),
            },
            self.color,
            2,    
        );
    }

    pub fn to_internal(&self) -> Polyline<f32> {
        Polyline::new(self.internal.points().to_vec(), None)
    }

    pub fn points(&self) -> Vec<Point> {
        self.internal.points().iter().map(|np| Point::new(np.x, np.y)).collect()
    }
}

use coffee::{
    graphics::{
        Point,
        Shape,
        Rectangle,

    }
};

use ncollide2d::{
    shape::Polyline,
    math::Point as NPoint,
};

pub trait ShapeExt {

    fn rect(tl: Point, width: f32, height: f32) -> Shape {
        Shape::Rectangle(Rectangle {
            x: tl.x,
            y: tl.y,
            width,
            height,
        })
    }

    fn rect_from_corners(tl: Point, tr: Point, _bl: Point, br: Point) -> Shape {
        Shape::Rectangle(Rectangle {
            x: tl.x,
            y: tl.y,
            width: (tr.x - tl.x).abs(),
            height: (br.y - tr.y).abs(),
        })
    }

    fn rect_from_center(center: Point, width: f32, height: f32) -> Shape {
        Shape::Rectangle(Rectangle {
            x: center.x - width / 2.0,
            y: center.y - height / 2.0,
            width,
            height,
        })
    }

    fn triangle(a: Point, b: Point, c: Point) -> Shape {
        Shape::Polyline {
            points: vec![a, b, c]
        }
    }
}

impl ShapeExt for Shape {}

pub fn polyline_from_shape(shape: Shape) -> Polyline<f32> {
    match shape {
        Shape::Rectangle(Rectangle {
            x,
            y,
            width,
            height
        }) => {
            let points = vec![
                NPoint::new(x, y),
                NPoint::new(x + width, y),
                NPoint::new(x + width, y + height),
                NPoint::new(x, y + height),
            ];
            Polyline::new(points, None)
        },
        Shape::Polyline { points } => {
            let points = points.iter().map(|point| NPoint::new(point.x, point.y)).collect::<Vec<NPoint<f32>>>();
            Polyline::new(points, None)
        },
        _ => unreachable!()

    }
}
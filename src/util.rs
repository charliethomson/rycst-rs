

use std::f32::consts::PI;
use coffee::graphics::Point;

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub fn to_degs(rad: f32) -> f32 {
    rad * (180.0 / PI)
}

pub fn to_rads(deg: f32) -> f32 {
    deg * (PI / 180.0)
}

pub fn remap(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    ((value - old_min) * (new_max - new_min)) / (old_max - old_min) + new_min
}

pub fn max_vals_points(points: &Vec<Point>) -> (i64, i64) {
    points.iter().fold((std::i64::MAX, std::i64::MAX), |(accx, accy), point| {
        let (px, py) = (point.x.round() as i64, point.y.round() as i64);
        match (accx, accy) {
            (std::i64::MAX, std::i64::MAX) => {
                (px, py) 
            },
            (std::i64::MAX, _) => {
                if py < accy {
                    (px, accy)
                } else {
                    (px, py)
                }
            },
            (_, std::i64::MAX) => {
                if px < accx {
                    (accx, py)
                } else {
                    (px, py)
                }
            },
            (_, _) => {

                let a = if px < accx {
                    accx
                } else {
                    px
                };

                let b = if py < accy {
                    accy
                } else {
                    py
                };

                (a, b)
            }
        }
    })
}

#[test]
fn test_degs_rads() {
    let a = 10.0; 
    let b = to_degs(to_rads(a));
    assert_eq!(a.round(), b.round());
}

#[test]
fn test_max_vals_points() {
    let points = vec![
        Point::new(50.0, 0.0),
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(0.0, 10.0),
        Point::new(0.0, 0.0),
        Point::new(0.0, 0.0),
        Point::new(0.0, 110.0),
        Point::new(10.0, 0.0),
    ];

    let (mx, my) = max_vals_points(&points);
    assert_eq!(mx, 50);
    assert_eq!(my, 110);
}
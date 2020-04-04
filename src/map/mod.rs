
pub(crate) mod wall;
pub(crate) use wall::Wall;
use coffee::graphics::{
    Rectangle,
    Mesh,
    Color,
    Shape,
    Point,
};

use crate::{
    config::MAX_TOI,
    util::{
        remap,
    },
};


use std::{
    fs::File,
    path::Path,
    io::Read,
};

use ncollide2d::{
    math::{
        Point as NPoint,
        Isometry,
    },
    shape::Polyline,
    query::{ Ray, RayCast, RayIntersection, },
};

pub struct Map {
    pub walls: Vec<Wall>,
} impl Map {
    pub fn new(sz: (f32, f32)) -> Self {
        Self {
            walls: vec![],
        }
    }

    pub fn debug() -> Self {
        let walls = vec![
            // borders
            Wall::new(vec![NPoint::new(0.0, 0.0), NPoint::new(0.0, 100.0)], Color::from_rgb(255, 0, 0)),
            Wall::new(vec![NPoint::new(0.0, 100.0), NPoint::new(100.0, 100.0)], Color::from_rgb(255, 0, 0)),
            Wall::new(vec![NPoint::new(100.0, 100.0), NPoint::new(100.0, 0.0)], Color::from_rgb(255, 0, 0)),
            Wall::new(vec![NPoint::new(100.0, 0.0), NPoint::new(0.0, 0.0)], Color::from_rgb(255, 0, 0)),
            // vertical cross
            Wall::new(vec![NPoint::new(50.0, 25.0), NPoint::new(50.0, 75.0)], Color::from_rgb(0, 255, 0)),
            Wall::new(vec![NPoint::new(25.0, 50.0), NPoint::new(75.0, 50.0)], Color::from_rgb(0, 255, 0)),
            // 45 degree cross
            Wall::new(vec![NPoint::new(25.0, 25.0), NPoint::new(75.0, 75.0)], Color::from_rgb(0, 0, 255)),
            Wall::new(vec![NPoint::new(25.0, 75.0), NPoint::new(75.0, 25.0)], Color::from_rgb(0, 0, 255)),
        ];

        Self {
            walls,
        }
    }

    pub fn convex() -> Self {
        let walls = vec![
            // borders
            Wall::new(vec![NPoint::new(250.0, 375.0), NPoint::new(500.0, 200.0)], Color::from_rgb(255, 127, 0  )),
            Wall::new(vec![NPoint::new(500.0, 200.0), NPoint::new(750.0, 625.0)], Color::from_rgb(0  , 255, 127)),
            Wall::new(vec![NPoint::new(750.0, 625.0), NPoint::new(750.0, 750.0)], Color::from_rgb(127, 0  , 255)),
            Wall::new(vec![NPoint::new(750.0, 750.0), NPoint::new(500.0, 870.0)], Color::from_rgb(255, 0  , 127)),
            Wall::new(vec![NPoint::new(500.0, 870.0), NPoint::new(250.0, 500.0)], Color::from_rgb(127, 255, 0  )),
            Wall::new(vec![NPoint::new(250.0, 500.0), NPoint::new(250.0, 375.0)], Color::from_rgb(0  , 127, 255)),
        ];

        Self {
            walls,
        }
    }

    fn as_polyline(&self) -> Polyline<f32> {
        Polyline::new(
            self.walls
                .iter()
                .fold(vec![], |mut acc, wall| {
                    acc.extend_from_slice(&wall.npoints());
                    acc
                })
            ,
            None
        )
    }

    pub fn center_point(&self) -> NPoint<f32> {
        let poly = self.as_polyline();
        let aabb = poly.aabb();
        
        aabb.center()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Encountered an error opening map file: {}", err))
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("Encountered an error reading from map file: {}", e))
        }

        // TODO: Design map file format, implement parsing here: SERDE MAYBE

        Err("Unimplemented!".to_owned())
    }

    pub fn push_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }

    pub fn dims(&self) -> (NPoint<f32>, NPoint<f32>) {
        let poly = self.as_polyline();
        (poly.aabb().mins().clone(), poly.aabb().maxs().clone())
    }

    pub fn draw_top_down(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = Mesh::new();

        let (mins, maxs) = self.dims();
        for wall in self.walls.iter() {
            mesh.stroke(
                Shape::Polyline {
                    points: wall.points()
                                .iter()
                                .map(|point| {
                                    Point::new(
                                        remap(point.x, mins.x, maxs.x, window.x, window.width),
                                        remap(point.y, mins.y, maxs.y, window.y, window.height),
                                    )
                                })
                                .collect::<Vec<Point>>()
                },
                wall.color,
                1,
            )
        }
        
        mesh
    }

    fn nearest_collision(collisions: Vec<(Option<RayIntersection<f32>>, &Wall)>) -> Option<(RayIntersection<f32>, &Wall)> {
        collisions
            .iter()
            .filter(|(op, _)| op.is_some())
            .map(|(op, w)| (op.unwrap(), w))
            .filter(|(inter, _)| inter.toi <= MAX_TOI)
            .fold(None, |acc, (v, w)| {
                if let Some((accv, _)) = acc {
                    if v.toi < accv.toi {
                        Some((v, w))
                    } else {
                        Some((accv, w))
                    }
                } else {
                    Some((v, w))
                }
            })
    }

    pub fn get_collision(&self, ray: &Ray<f32>, m: &Isometry<f32>) -> Option<(RayIntersection<f32>, &Wall)> {
        let mut collisions: Vec<(Option<RayIntersection<f32>>, &Wall)> = vec![];
        for wall in self.walls.iter() {
            collisions.push(
                (
                    wall
                    .to_internal()
                    .toi_and_normal_with_ray(
                        m,
                        ray,
                        MAX_TOI,
                        true, // TODO: Test true/false for speed / accuracy
                    ),
                    wall
                )
            )
        }

        Self::nearest_collision(collisions)
    }
}

#[test]
fn test_nearest_collision() {
    use ncollide2d::shape::FeatureId;
    use ncollide2d::math::Vector;
    let null_wall = Wall::new(vec![
        NPoint::new(0.0, 0.0),
        NPoint::new(10.0, 10.0),
    ], Color::WHITE);
    let collisions: Vec<(Option<RayIntersection<f32>>, &Wall)> = vec![
        50.0,
        120.0,
        10.5,
        10.0,
        11.0,
        500.0,
        30.0,
    ].iter()
    .map(|toi| RayIntersection::new(*toi as f32, Vector::new_random(), FeatureId::Unknown))
    .map(|inter| (Some(inter), &null_wall))
    .collect();

    collisions.iter().for_each(|col| eprintln!("{:?}", col.0));

    assert_eq!(10.0, Map::nearest_collision(collisions).unwrap().0.toi)
}
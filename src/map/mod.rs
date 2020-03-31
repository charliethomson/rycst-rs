
pub(crate) mod wall;
pub(crate) use wall::Wall;
use coffee::graphics::{
    Rectangle,
    Mesh,
    Color,
};

use crate::config::MAX_TOI;

use std::{
    fs::File,
    path::Path,
    io::Read,
    collections::HashMap,
    convert::TryInto,
};

use ncollide2d::{
    math::Point as NPoint,
    query::{ Ray, RayCast, RayIntersection, },
    math::Isometry,
};

pub struct Map {
    walls: Vec<Wall>,
    sz: (f32, f32),
} impl Map {
    pub fn new(sz: (f32, f32)) -> Self {
        Self {
            walls: vec![],
            sz
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
            sz: (100.0, 100.0),
        }
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

    pub fn dims(&self) -> (f32, f32) {
        self.sz
    }

    pub fn draw_top_down(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = Mesh::new();

        for wall in self.walls.iter() {
            wall.draw_to_mesh(&mut mesh, window)
        }
        
        mesh
    }

    fn nearest_collision(collisions: Vec<Option<RayIntersection<f32>>>) -> Option<RayIntersection<f32>> {
        let mut cols: HashMap<i128, RayIntersection<f32>> = HashMap::new();
        for col in collisions {
            if let Some(col) = col {
                
                let RayIntersection { toi, .. } = col;
                cols.insert(
                    toi.round() as i128,
                    col,
                );
            } else { continue }
        }

        if cols.is_empty() {
            None
        } else {
            let key = cols.iter().fold(/* min */ std::i128::MAX, |min, cur| {
                if *cur.0 < min {
                    *cur.0
                } else {
                    min
                }
            });

            if let Some(rf) = cols.get(&key) {
                Some(rf.clone())
            } else {
                None
            }
        }
    }

    pub fn ray_collides_with(&self, ray: &Ray<f32>, m: &Isometry<f32>) -> Option<RayIntersection<f32>> {
        let mut collisions: Vec<Option<RayIntersection<f32>>> = vec![];
        for wall in self.walls.iter() {
            collisions.push(
                wall
                    .to_internal()
                    .toi_and_normal_with_ray(
                        m,
                        ray,
                        MAX_TOI,
                        true, // TODO: Test true/false for speed / accuracy
                    )
            )
        }

        Self::nearest_collision(collisions)
    }
}
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

use crate::math::{vector::Vec2D, wall::Wall};

#[derive(Debug)]
pub enum MapError {
    Io(std::io::Error),
    Json(serde_json::Error),
}
impl From<std::io::Error> for MapError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for MapError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapFile {
    dimensions: (u32, u32),

    walls: Vec<MapFileWall>,
}

#[derive(Serialize, Deserialize)]
pub struct MapFileWall {
    wall_color: usize,

    start: MapFilePoint,

    end: MapFilePoint,
}

impl From<MapFileWall> for Wall {
    fn from(map_wall: MapFileWall) -> Self {
        Wall::new(
            map_wall.start.into(),
            map_wall.end.into(),
            map_wall.wall_color,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapFilePoint {
    x: f64,
    y: f64,
}
impl From<MapFilePoint> for Vec2D {
    fn from(this: MapFilePoint) -> Self {
        Vec2D::new(this.x, this.y)
    }
}

pub struct Map {
    pub walls: Vec<Wall>,
    pub dims: (u32, u32),
}
impl Map {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, MapError> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let file_parsed: MapFile = serde_json::from_str(&contents)?;

        let walls: Vec<Wall> = file_parsed.walls.into_iter().map(Wall::from).collect();

        println!(
            "Successfully loaded {} walls from \"{}\"",
            walls.len(),
            path.as_ref().to_str().unwrap()
        );

        Ok(Self {
            walls,
            dims: file_parsed.dimensions,
        })
    }
}
impl Default for Map {
    fn default() -> Self {
        Self::load("./assets/maps/standard.json").unwrap()
    }
}

use geo::Line;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

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

#[derive(Clone)]
pub struct Wall {
    pub color_index: usize,
    pub line: Line<f64>,
}
impl From<Wall> for Line<f64> {
    fn from(w: Wall) -> Self {
        w.line
    }
}
impl From<MapFileWall> for Wall {
    fn from(wall: MapFileWall) -> Self {
        Wall {
            color_index: wall.wall_color,
            line: Line::new(wall.start.into(), wall.end.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapFile {
    name: String,

    dimensions: (u32, u32),

    walls: Vec<MapFileWall>,
}

#[derive(Serialize, Deserialize)]
pub struct MapFileWall {
    wall_color: usize,

    start: MapFilePoint,

    end: MapFilePoint,
}

#[derive(Serialize, Deserialize)]
pub struct MapFilePoint {
    x: f64,
    y: f64,
}
impl From<MapFilePoint> for geo::Point<f64> {
    fn from(this: MapFilePoint) -> Self {
        geo::Point::new(this.x, this.y)
    }
}

pub struct Map {
    pub walls: Vec<Wall>,
    pub dims: (u32, u32),
}
impl Map {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, MapError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let file_parsed: MapFile = serde_json::from_str(&contents)?;

        println!("Successfully loaded \"{}\"", file_parsed.name);

        let walls = file_parsed.walls.into_iter().map(Wall::from).collect();

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

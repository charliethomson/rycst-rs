
pub(crate) mod ext;

pub use ext::ShapeExt;

use crate::{
    config::{
        MAX_TOI,
        RENDER_FOV,
        RENDER_RESOLUTION,
        MVMT_SPD,
        ROTATE_SPD
    },
    map::{ Map, Wall },
    util::{
        Direction,
        to_rads,
        to_degs,
        remap,
    },
};

use std::f32::consts::FRAC_PI_2;

use ncollide2d::{
    query::{ Ray, RayCast, RayIntersection },
    math::{ Vector, Point as NPoint, Isometry, },
};

use nalgebra::{
    geometry::UnitComplex
};

use coffee::{
    graphics::{
        Mesh,
        Rectangle,
        Shape,
        Color,
        Point,
    },
};



pub struct Renderer {
    camera: Ray<f32>,
    map: Map,
} impl Renderer {
    pub fn new() -> Self {
        let map = Map::convex();
        let (x, y) = map.dims();
        let camera = Ray::new( map.center_point(), Vector::new(0.0, 10.0) );
        Self {
            camera,
            map,
        }
    }
    
    pub fn with_map(map: Map) -> Self {
        let mut r = Self::new();
        r.map = map;
        r
    }

    pub fn mv(&mut self, dir: Direction) {
        self.camera = match dir {
            Direction::Forward => {
                self.camera.transform_by(
                    &Isometry::translation(
                        MVMT_SPD * self.angle().cos(),
                        MVMT_SPD * self.angle().sin(),
                    )
                )
            },
            Direction::Backward => {
                self.camera.inverse_transform_by(
                    &Isometry::translation(
                        MVMT_SPD * self.angle().cos(),
                        MVMT_SPD * self.angle().sin(),
                    )
                )
            },
            Direction::Right => {
                self.camera.transform_by(
                    &Isometry::translation(
                        MVMT_SPD * (self.angle() - FRAC_PI_2).cos(),
                        MVMT_SPD * (self.angle() - FRAC_PI_2).sin(),
                    )
                )
            },
            Direction::Left => {
                self.camera.inverse_transform_by(
                    &Isometry::translation(
                        MVMT_SPD * (self.angle() - FRAC_PI_2).cos(),
                        MVMT_SPD * (self.angle() - FRAC_PI_2).sin(),
                    )
                )
            },
        };
    }

    pub fn rotate(&mut self, dir: Direction) {
        let dir = match dir {
            Direction::Left => 1.0,
            Direction::Right => -1.0,
            _ => 0.0,
        };

        let amt = dir * ROTATE_SPD;
        let iso = Isometry::rotation(amt);
        self.camera.dir = self.camera.transform_by(&iso).dir;
    }

    pub fn angle(&self) -> f32 {
        let x = self.camera.dir.get(0).expect("This shouldn't be possible, failed to get the x component of vector in the Renderer.angle function");
        let y = self.camera.dir.get(1).expect("This shouldn't be possible, failed to get the y component of vector in the Renderer.angle function");

        y.atan2(*x)
    }

    pub fn angle_degrees(&self) -> f32 {
        to_degs(self.angle())
    }

    pub fn render_top_down(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = Mesh::new();

        let (mins, maxs) = self.map.dims();
        let mapped_position = Point::new(
            remap(self.camera.origin.x, mins.x, maxs.x, window.x, window.width),
            remap(self.camera.origin.y, mins.y, maxs.y, window.y, window.height),
        );
        
        for wall in self.map.walls.iter() {
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
        
        mesh.fill(
            Shape::rect_from_center(
                mapped_position,
                2.0,
                2.0,
            ),
            Color::WHITE,
        );

        mesh.stroke(
            Shape::Polyline {
                points: vec![
                    Point::new(mapped_position.x, mapped_position.y),
                    Point::new(
                        mapped_position.x + (MAX_TOI * self.angle().cos()),
                        mapped_position.y + (MAX_TOI * self.angle().sin()),
                    ),
                    Point::new(mapped_position.x, mapped_position.y),
                    Point::new(
                        mapped_position.x + (MAX_TOI * (self.angle() - (to_rads(RENDER_FOV) / 2.0)).cos()),
                        mapped_position.y + (MAX_TOI * (self.angle() - (to_rads(RENDER_FOV) / 2.0)).sin()),
                    ),
                    Point::new(mapped_position.x, mapped_position.y),
                    Point::new(
                        mapped_position.x + (MAX_TOI * (self.angle() + (to_rads(RENDER_FOV) / 2.0)).cos()),
                        mapped_position.y + (MAX_TOI * (self.angle() + (to_rads(RENDER_FOV) / 2.0)).sin()),
                    ),
                ]
            },
            Color::WHITE,
            1,
        );

        mesh.stroke(Shape::Rectangle(window.clone()), Color::from_rgb(255, 0, 0), 2);

        mesh
    }

    pub fn render(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = Mesh::new();

        let step_sz: f32 = to_rads(RENDER_FOV / RENDER_RESOLUTION);
        let sector_width: f32 = window.width / RENDER_RESOLUTION;
        let mut ray = self.camera.clone();
        // Create isometry interface, we use this to transform the ray
        // let mut iso = Isometry::translation(ray.origin.x, ray.origin.y);
        let mut iso = Isometry::rotation(self.angle() - RENDER_FOV / 2.0);
        // Initialize the ray to the left side of the camera's fov
        // iso.append_rotation_mut(&UnitComplex::new(self.angle() - RENDER_FOV / 2.0));
        ray.transform_by(&iso);

        // ray is currently at the left side of the fov
        let mut offset_angle = 0.0;
        // index of the current ray
        let mut offset = 0;

        while offset_angle < to_rads(RENDER_FOV) {
            // Cast a ray, get the intersection

            // We need a transform for the polyline for the raycast function, we use the offset angle to do that
            let cast_iso = Isometry::rotation(offset_angle);
            
            match self.map.get_collision(&ray, &cast_iso) {
                Some((intersection, with_wall)) => {
                    // map the TOI to the height of the sector
                    let RayIntersection { toi, .. } = intersection;

                    // correct the distance from the wall (w/o this, fisheye happens)
                    let corrected_toi = toi * offset_angle.cos();
                    let sector_height = remap(corrected_toi, 0.0, MAX_TOI, window.height, 0.0);
                    
                    // create the sector shape

                    // we need the position of the top left corner of the sector for the constructor:
                    let tl = Point::new(
                        // the left edge of the sector is the sector width * the current offset 
                        window.x + (sector_width * offset as f32),
                        // the top of the sector is (the height of the window - the height of the sector) / 2
                        window.y + ((window.height - sector_height) / 2.0),
                    );

                    let sector = Shape::rect(tl, sector_width, sector_height);
                    
                    // add the sector to the mesh
                    mesh.fill(sector, with_wall.color); 
                },
                None => {
                    // ()
                    mesh.fill(
                        Shape::rect(
                            Point::new(window.x + (offset as f32 * sector_width), window.y),
                            sector_width,
                            window.height,
                        ),
                        Color::WHITE
                    );
                },
            }

            // Advance the offset angle at the end of the loop so we're not ahead by one sector
            offset_angle += step_sz;
            offset += 1;
        }
        
        mesh.stroke(Shape::Rectangle(window.clone()), Color::from_rgb(0, 255, 0), 5);

        mesh
    }
}

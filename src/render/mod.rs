
pub(crate) mod ext;

pub use ext::ShapeExt;
use crate::map::{ Map, Wall };
use crate::config::{
    MAX_TOI,
    RENDER_FOV,
    RENDER_RESOLUTION,
    WIDTH, 
    HEIGHT
};


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
        let map = Map::debug();
        let (x, y) = map.dims();
        let camera = Ray::new( NPoint::new(0.1 * x, 0.8 * y), Vector::new_random() );
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

    pub fn rotate(&mut self, amt: f32) {
        let iso = Isometry::rotation(amt / 10.0);
        self.camera.transform_by(&iso);
    }

    // FIXME: This is not right, please fix. Im going to bed. This doesn't particularly work yet lol, working on it :)
    pub fn angle(&mut self) -> f32 {
        self.camera.dir.angle(&self.camera.dir)
    }

    pub fn render_top_down(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = self.map.draw_top_down(window);

        mesh.fill(Shape::rect(Point::new(self.camera.origin.x + window.x, self.camera.origin.y + window.y), 4.0, 4.0), Color::WHITE);

        mesh
    }

    fn remap(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
        ((value - old_min) * (new_max - new_min)) - (old_max - old_min)
    }

    pub fn render(&self, window: &Rectangle<f32>) -> Mesh {
        let mut mesh = Mesh::new();

        let step_sz: f32 = RENDER_FOV / RENDER_RESOLUTION;
        let sector_width: f32 = window.width / RENDER_RESOLUTION;
        let mut ray = self.camera.clone();
        // Create isometry interface, we use this to transform the ray
        let mut iso = Isometry::translation(ray.origin.x, ray.origin.y);
        // Initialize the ray to the left side of the camera's fov
        iso.append_rotation_mut(&UnitComplex::new(ray.dir.angle(&Vector::zeros()) - RENDER_FOV / 2.0));
        ray.transform_by(&iso);

        // ray is currently at the left side of the fov
        let mut offset_angle = 0.0;

        for offset in 0..RENDER_RESOLUTION as usize {
            // Cast a ray, get the intersection

            // We need a transform for the polyline for the raycast function, we use the offset angle to do that
            let cast_iso = Isometry::rotation(offset_angle);
            
            match self.map.ray_collides_with(&ray, &cast_iso) {
                Some(intersection) => {
                    // map the TOI to the height of the sector
                    let RayIntersection { toi, .. } = intersection;
                    let sector_height = Self::remap(toi, 0.0, MAX_TOI, 0.0, window.height);
                    
                    // create the sector shape

                    // we need the position of the top left corner of the sector for the constructor:
                    let tl = Point::new(
                        // the top of the sector is (the height of the window - the height of the sector) / 2
                        (window.height - sector_height) / 2.0,
                        // the left edge of the sector is the sector width * the current offset 
                        sector_width * offset as f32
                    );

                    let sector = Shape::rect(tl, sector_width, sector_height);
                    
                    // add the sector to the mesh
                    mesh.fill(sector, Color::WHITE); // TODO: implement colors for the walls and such
                },
                None => (),
            }

            // Advance the offset angle at the end of the loop so we're not ahead by one sector
            offset_angle += step_sz;
        }
        

        mesh
    }
}
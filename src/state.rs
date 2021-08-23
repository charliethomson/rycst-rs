use std::collections::HashMap;

use num_traits::{AsPrimitive, Float};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    ext::ColorExt, key_state_handler::KeyStateHandler, map::Map, math::vector::Vec2D,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub fn remap<
    T: 'static + Float + Copy,
    ON: AsPrimitive<T> + Copy,
    OX: AsPrimitive<T> + Copy,
    NN: AsPrimitive<T> + Copy,
    NX: AsPrimitive<T> + Copy,
>(
    value: T,
    old_min: ON,
    old_max: OX,
    new_min: NN,
    new_max: NX,
) -> T {
    new_min.as_()
        + (new_max.as_() - new_min.as_())
            * ((value - old_min.as_()) / (old_max.as_() - old_min.as_()))
}

pub fn remap_minz<
    T: 'static + Float + Copy + AsPrimitive<T>,
    OX: AsPrimitive<T> + Copy,
    NX: AsPrimitive<T> + Copy,
>(
    value: T,
    old_max: OX,
    new_max: NX,
) -> T {
    remap(value, T::zero(), old_max, T::zero(), new_max)
}

const MOUSE_SENSITIVITY: f64 = 0.01;
const MOVE_SPEED: f64 = 2.5;
const PLAYER_WALL_PADDING: f64 = 10.;
const WALL_ACTUAL_HEIGHT: f64 = 48.;

pub struct State {
    pub(crate) position: Vec2D,
    pub(crate) angle: f64,
    pub(crate) fov: f64,
    pub(crate) wall_colors: Vec<Color>,
    pub(crate) map: Map,
    pub(crate) keys: KeyStateHandler,
    pub(crate) columns: Vec<(usize, u32)>,
    pub(crate) resolution: usize,
    pub(crate) projection_factor: f64,
    pub(crate) radian_per_column: f64,
    pub(crate) column_width: u32,
}
impl State {
    pub fn new() -> Self {
        let wall_colors = vec![
            Color::RGB(128, 255, 0),
            Color::RGB(0, 128, 255),
            Color::RGB(255, 0, 128),
            Color::RGB(0, 255, 0),
            Color::RGB(0, 0, 255),
            Color::WHITE,
        ];

        // let map = Map::default();
        let map = Map::load("./assets/maps/many_walls.json").unwrap();
        let (w, h) = map.dims;
        // let movement_vector = Line::new(origin, origin + geo::Point::new(delta_x, delta_y));
        let position = Vec2D::new(w as f64 / 2., 50. + h as f64 / 2.);
        let fov = 60.;
        let projection_plane_distance =
            ((WINDOW_WIDTH / 2) as f64 / (fov.to_radians() / 2.).tan()) as f64;

        let resolution = WINDOW_WIDTH as usize;

        Self {
            position,
            angle: std::f64::consts::PI,
            fov,
            wall_colors,
            map,
            keys: KeyStateHandler::new(),
            columns: Vec::with_capacity(resolution),
            resolution,
            projection_factor: projection_plane_distance * WALL_ACTUAL_HEIGHT,
            radian_per_column: fov.to_radians() / resolution as f64,
            column_width: WINDOW_WIDTH / resolution as u32,
        }
    }

    fn get_color(&self, index: usize) -> Color {
        self.wall_colors.get(index).copied().unwrap_or(Color::WHITE)
    }

    pub fn mouse_motion(&mut self, dx: i32) {
        self.angle += MOUSE_SENSITIVITY * dx as f64;
    }

    fn calculate_collisions(&mut self) {
        let mut current_angle = self.angle - (self.fov.to_radians() / 2.);
        let end_angle = current_angle + (self.radian_per_column * self.resolution as f64);

        self.columns.clear();

        for _ in 0..self.resolution {
            let mut ray = Vec2D::from_angle(current_angle);
            ray.translate(&self.position);

            let mut max_height = f64::NEG_INFINITY;
            let mut wall_color_index = 0;

            for wall in self.map.walls.iter() {
                if let Some(intersection_vector) = ray.intersects(wall) {
                    let raw_distance = ray.dist(&intersection_vector);
                    let delta = current_angle - self.angle;
                    let corrected_distance = raw_distance * (delta.cos() as f64);
                    let projected_height = self.projection_factor / corrected_distance;

                    if projected_height > max_height {
                        max_height = projected_height;
                        wall_color_index = wall.color_index;
                    }
                }
            }
            if max_height.is_infinite() {
                self.columns.push((0, 0));
            } else {
                self.columns
                    .push((wall_color_index, max_height.round() as u32));
            }

            current_angle += self.radian_per_column;
        }
    }

    fn update_camera(&mut self) {
        let mut delta = Vec2D::Origin;

        let par = Vec2D::from_angle(self.angle as f64);
        let perp = Vec2D::from_angle(self.angle + (90f64).to_radians());

        if self.keys.is_pressed(Keycode::W) {
            delta += par;
        }
        if self.keys.is_pressed(Keycode::S) {
            delta += -par;
        }
        if self.keys.is_pressed(Keycode::A) {
            delta += -perp;
        }
        if self.keys.is_pressed(Keycode::D) {
            delta += perp;
        }

        // Normalize delta so that the player doesn't move faster moving in a diagonal direction
        if !delta.is_origin() {
            delta = delta.normalize() * MOVE_SPEED;
        }

        self.position.add_x_y_raw(delta.x_y());
        self.position.clamp(self.map.dims, PLAYER_WALL_PADDING);
    }

    pub fn update(&mut self) {
        self.update_camera();
        self.calculate_collisions();
    }

    pub fn draw_minimap(
        &self,
        canvas: &mut Canvas<Window>,
        dims: (f64, f64),
    ) -> Result<(), String> {
        let minimap_offset = (dims.0.max(dims.1) / 4.);
        let minimap_base = Vec2D::new(minimap_offset, minimap_offset);
        // Background
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(
            minimap_offset as i32,
            minimap_offset as i32,
            dims.0 as u32,
            dims.1 as u32,
        ))?;

        canvas.set_draw_color(Color::WHITE);

        // Player position
        let position_mapped = self.position.remap(self.map.dims, dims) + minimap_base;
        canvas.fill_rect(Rect::from_center(position_mapped, 8, 8))?;

        // Player lines
        let ray_scale = dims.0.max(dims.1) / 2.;
        let half_fov = self.fov.to_radians() / 2.;

        let forward_end = position_mapped + (Vec2D::from_angle(self.angle) * ray_scale);
        let left_end = position_mapped + (Vec2D::from_angle(self.angle - half_fov) * ray_scale);
        let right_end = position_mapped + (Vec2D::from_angle(self.angle + half_fov) * ray_scale);

        canvas.draw_lines(&[
            position_mapped.into(),
            forward_end.into(),
            position_mapped.into(),
            left_end.into(),
            position_mapped.into(),
            right_end.into(),
        ] as &[Point])?;

        // TODO: FOV lines

        // Walls
        for wall in self.map.walls.iter() {
            canvas.set_draw_color(self.get_color(wall.color_index));

            let start = wall.a.remap(self.map.dims, dims) + minimap_base;
            let end = wall.b.remap(self.map.dims, dims) + minimap_base;

            canvas.draw_line(start, end)?;
        }

        // let mut current_angle = self.angle + (self.fov.to_radians() / 2.);

        // for _ in 0..self.resolution {
        //     let mut ray = self.position;
        //     ray.set_angle(current_angle);
        //     ray += self.position;

        //     let mut max_height = f64::NEG_INFINITY;
        //     let mut collisions: Vec<(bool, Vec2D)> = vec![];
        //     let mut collision = Vec2D::Origin;

        //     for wall in self.map.walls.iter() {
        //         if let Some(intersection_vector) = ray.intersects(wall) {
        //             let raw_distance = ray.dist(&intersection_vector);
        //             let delta = current_angle - self.angle;
        //             let corrected_distance = raw_distance * (delta.cos() as f64);
        //             let projected_height = self.projection_factor / corrected_distance;

        //             if projected_height > max_height {
        //                 max_height = projected_height;
        //                 collisions = collisions.into_iter().fold(vec![], |mut acc, cur| {
        //                     acc.push((false, cur.1));
        //                     acc
        //                 });
        //                 collisions.push((true, collision));
        //                 collision = intersection_vector;
        //             } else {
        //                 collisions.push((false, intersection_vector));
        //             }
        //         }
        //     }
        //     if !max_height.is_infinite() {
        //         canvas.set_draw_color(Color::RED);
        //         canvas.draw_rects(
        //             collisions
        //                 .into_iter()
        //                 .map(|(_, v)| {
        //                     Rect::from_center(
        //                         Point::new(
        //                             remap(v.x(), 0., self.map.dims.0, 0., dims.0).floor() as i32,
        //                             remap(v.y(), 0., self.map.dims.1, 0., dims.1).floor() as i32,
        //                         ) + minimap_base,
        //                         2,
        //                         2,
        //                     )
        //                 })
        //                 .collect::<Vec<Rect>>()
        //                 .as_slice(),
        //         );
        //     }

        //     current_angle -= self.radian_per_column;
        // }

        Ok(())
    }

    fn render_frame(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let column_width_signed = self.column_width as i32;

        // TODO: Draw background

        let mut current_index = usize::MAX;
        let mut current_color = Color::BLACK;

        for (idx, (color_index, height)) in self.columns.iter().copied().enumerate() {
            if color_index != current_index {
                current_color = self.get_color(color_index);
                current_index = color_index;
            }

            let dim_amt = remap(height as f64, 0, WINDOW_HEIGHT, 255, 0).floor() as u8;

            canvas.set_draw_color(current_color.dim(dim_amt));
            canvas.fill_rect(Rect::from_center(
                Point::new(
                    idx as i32 * column_width_signed + (column_width_signed / 2),
                    WINDOW_HEIGHT as i32 / 2,
                ),
                self.column_width,
                height,
            ))?;
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.render_frame(canvas)?;
        self.draw_minimap(canvas, (WINDOW_WIDTH as f64 / 5., WINDOW_WIDTH as f64 / 5.))?;
        Ok(())
    }
}

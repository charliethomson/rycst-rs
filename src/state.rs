use std::collections::HashMap;

use geo::{algorithm::distance::Distance, Line};
use line_intersection::LineInterval;
use num_traits::{AsPrimitive, Float};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    app::get_canvas_size,
    map::{Map, Wall},
    WINDOW_WIDTH,
};

fn remap<
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

trait ColorExt {
    fn dim(&self, by: u8) -> Self;
}

impl ColorExt for Color {
    fn dim(&self, by: u8) -> Self {
        Self::RGB(
            self.r.saturating_sub(by),
            self.g.saturating_sub(by),
            self.b.saturating_sub(by),
        )
    }
}

const MOUSE_SENSITIVITY: f32 = 0.01;
const MOVE_SPEED: f32 = 2.5;
const PLAYER_WALL_PADDING: f32 = 10.;

pub struct State {
    pos: (f32, f32),
    angle: f32,
    fov: f32,
    render_dist: f32,
    wall_colors: Vec<Color>,
    map: Map,
    keys: HashMap<Keycode, bool>,
    collision_columns: Vec<(usize, u32)>,
    pub resolution: usize,
    debug: Vec<(usize, bool, geo::Point<f64>)>,
}
impl State {
    pub fn new() -> Self {
        let map = Map::default();
        let (w, h) = map.dims;
        let pos = (w as f32 / 2., 50. + h as f32 / 2.);
        let wall_colors = vec![
            // Color::RGB(0, 0, 0),
            // Color::RGB(128, 255, 0),
            // Color::RGB(0, 128, 255),
            // Color::RGB(255, 0, 128),
            // Color::RGB(0, 255, 0),
            // Color::RGB(0, 0, 255),
            Color::WHITE,
        ];

        Self {
            pos,
            angle: 0.,
            fov: 60.,
            render_dist: 1000.,
            wall_colors,
            map,
            keys: HashMap::new(),
            collision_columns: vec![],
            resolution: WINDOW_WIDTH as usize,
            debug: vec![],
        }
    }

    fn get_color(&self, index: usize) -> Color {
        self.wall_colors.get(index).cloned().unwrap_or(Color::WHITE)
    }

    pub fn key_down(&mut self, key: Keycode) {
        self.keys.insert(key, true);
    }

    pub fn key_up(&mut self, key: Keycode) {
        self.keys.insert(key, false);
    }

    pub fn mouse_motion(&mut self, dx: i32) {
        self.angle -= MOUSE_SENSITIVITY * dx as f32;
    }

    fn calculate_collisions(&mut self) {
        let mut current_ray: LineInterval<f64>;

        let mut current_col = 0;
        let mut current_angle = self.angle + (self.fov.to_radians() / 2.);
        let radian_per_column = self.fov.to_radians() / self.resolution as f32;
        let origin = geo::Point::new(self.pos.0 as f64, self.pos.1 as f64);

        self.collision_columns = vec![];
        self.debug = vec![];

        let mut debug_index = 0;

        while current_col < self.resolution {
            current_ray = LineInterval::ray(Line::new(
                origin,
                geo::Point::new(
                    (self.pos.0 + current_angle.sin()) as f64,
                    (self.pos.1 + current_angle.cos()) as f64,
                ),
            ));

            let mut min_dist = f64::INFINITY;
            let mut wall_color_index = 0;
            let mut min_debug = 0;

            for Wall {
                color_index: wci,
                line: wall,
            } in self.map.walls.clone().into_iter()
            {
                let relation = current_ray.relate(&LineInterval::line_segment(wall));
                if let Some(intersection) = relation.unique_intersection() {
                    let raw_distance = origin.distance(&intersection);
                    let delta = current_angle - self.angle;
                    let corrected_distance = raw_distance * (delta.cos() as f64);

                    self.debug.push((debug_index, false, intersection));
                    if corrected_distance < min_dist {
                        min_dist = corrected_distance;
                        wall_color_index = wci;
                        min_debug = debug_index;
                    }
                    debug_index += 1;
                }
            }
            if min_dist.is_infinite() {
                self.collision_columns.push((0, 0));
            } else {
                self.collision_columns
                    .push((wall_color_index, min_dist.round() as u32));

                self.debug = self
                    .debug
                    .iter()
                    .copied()
                    .map(|(idx, min, intersection)| {
                        if idx == min_debug {
                            (idx, true, intersection)
                        } else {
                            (idx, min, intersection)
                        }
                    })
                    .collect();
            }

            current_col += 1;
            current_angle -= radian_per_column;
        }
    }

    fn handle_keys(&mut self) {
        let mut dx = 0.;
        let mut dy = 0.;

        let sx = self.angle.sin();
        let sy = self.angle.cos();
        let px = (self.angle + (90f32).to_radians()).sin();
        let py = (self.angle + (90f32).to_radians()).cos();

        if self.keys.get(&Keycode::W) == Some(&true) {
            dx += sx * MOVE_SPEED;
            dy += sy * MOVE_SPEED;
        }
        if self.keys.get(&Keycode::S) == Some(&true) {
            dx -= sx * MOVE_SPEED;
            dy -= sy * MOVE_SPEED;
        }
        if self.keys.get(&Keycode::A) == Some(&true) {
            dx += px * MOVE_SPEED;
            dy += py * MOVE_SPEED;
        }
        if self.keys.get(&Keycode::D) == Some(&true) {
            dx -= px * MOVE_SPEED;
            dy -= py * MOVE_SPEED;
        }

        self.pos.0 += dx;
        self.pos.1 += dy;

        // clamp player movement

        if self.pos.0 < PLAYER_WALL_PADDING {
            self.pos.0 = PLAYER_WALL_PADDING
        }
        if self.pos.1 < PLAYER_WALL_PADDING {
            self.pos.1 = PLAYER_WALL_PADDING
        }

        if self.pos.0 > self.map.dims.0 as f32 - PLAYER_WALL_PADDING {
            self.pos.0 = self.map.dims.0 as f32 - PLAYER_WALL_PADDING
        }
        if self.pos.1 > self.map.dims.1 as f32 - PLAYER_WALL_PADDING {
            self.pos.1 = self.map.dims.1 as f32 - PLAYER_WALL_PADDING
        }
    }

    pub fn update(&mut self) {
        // Key functions
        self.handle_keys();
        self.calculate_collisions();
    }

    pub fn draw_minimap(
        &self,
        canvas: &mut Canvas<Window>,
        dims: (f32, f32),
    ) -> Result<(), String> {
        let minimap_offset = (dims.0.max(dims.1) / 4.) as i32;
        // Background
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(
            minimap_offset,
            minimap_offset,
            dims.0 as u32,
            dims.1 as u32,
        ))?;

        canvas.set_draw_color(Color::WHITE);

        let player_pos_mapped: Point = (
            minimap_offset + (remap(self.pos.0, 0., self.map.dims.0, 0., dims.0).floor() as i32),
            minimap_offset + (remap(self.pos.1, 0., self.map.dims.1, 0., dims.1).floor() as i32),
        )
            .into();

        // Player position
        canvas.fill_rect(Rect::from_center(player_pos_mapped, 3, 3))?;

        // Player lines
        let ray_scale = remap(
            self.render_dist,
            0.,
            self.map.dims.0.max(self.map.dims.1),
            0.,
            dims.0.max(dims.1),
        );
        let forward_end: Point = (
            (player_pos_mapped.x + (ray_scale * self.angle.sin()) as i32),
            // .clamp(minimap_offset, minimap_offset + dims.0 as i32),
            (player_pos_mapped.y + (ray_scale * self.angle.cos()) as i32),
            // .clamp(minimap_offset, minimap_offset + dims.1 as i32),
        )
            .into();

        canvas.draw_line(player_pos_mapped, forward_end)?;

        // TODO: FOV lines

        // Walls
        for Wall { color_index, line } in self.map.walls.iter() {
            let ((start_raw_x, start_raw_y), (end_raw_x, end_raw_y)) = (
                (line.start.x(), line.start.y()),
                (line.end.x(), line.end.y()),
            );
            let start: Point = (
                minimap_offset
                    + (remap(start_raw_x, 0., self.map.dims.0, 0., dims.0).floor() as i32),
                minimap_offset
                    + (remap(start_raw_y, 0., self.map.dims.1, 0., dims.1).floor() as i32),
            )
                .into();
            let end: Point = (
                minimap_offset + (remap(end_raw_x, 0., self.map.dims.0, 0., dims.0).floor() as i32),
                minimap_offset + (remap(end_raw_y, 0., self.map.dims.1, 0., dims.1).floor() as i32),
            )
                .into();

            canvas.set_draw_color(self.get_color(*color_index));
            canvas.draw_line(
                Point::new(start.x as i32, start.y as i32),
                Point::new(end.x as i32, end.y as i32),
            )?;
        }

        for (_, min, intersection) in self.debug.iter() {
            if *min {
                canvas.set_draw_color(Color::GREEN);
            } else {
                canvas.set_draw_color(Color::RED);
            }
            let x = minimap_offset as f64 + remap(intersection.x(), 0, self.map.dims.0, 0, dims.0);
            let y = minimap_offset as f64 + remap(intersection.y(), 0, self.map.dims.1, 0, dims.1);
            canvas.fill_rect(Rect::from_center(Point::new(x as i32, y as i32), 4, 4))?;
        }

        Ok(())
    }

    fn render_frame(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let (window_width, window_height) = get_canvas_size(canvas);
        let column_width = window_width / self.collision_columns.len() as u32;
        if column_width == 0 {
            return Ok(());
        }

        // Draw background
        let num_rows = window_height / column_width;
        let top_col = Color::BLUE;
        let bottom_col = Color::RED;

        for row in 0..num_rows {
            if row < num_rows / 2 {
                let dim_amt = remap(row as f32, 0, num_rows / 2, 0, 255) as u8;
                canvas.set_draw_color(top_col.dim(dim_amt));
            } else {
                let dim_amt = remap(row as f32, num_rows / 2, num_rows, 255, 0) as u8;
                canvas.set_draw_color(bottom_col.dim(dim_amt));
            }
            canvas.fill_rect(Rect::new(
                0,
                (row * column_width) as i32,
                window_width,
                column_width,
            ))?;
        }

        let mut current_index = usize::MAX;
        let mut current_color = Color::BLACK;

        // Map height between 0 and window height
        let columns_normalized = self.collision_columns.iter().map(|(wci, height)| {
            (
                *wci,
                remap(*height as f32, 0., self.render_dist, window_height, 0.).round() as u32,
            )
        });

        for (idx, (color_index, height)) in columns_normalized.enumerate() {
            if color_index != current_index {
                current_color = self.get_color(color_index);
                current_index = color_index;
            }

            let dim_amt = remap(height as f32, 0, window_height, 255, 0).floor() as u8;

            canvas.set_draw_color(current_color.dim(dim_amt));
            canvas.fill_rect(Rect::from_center(
                Point::new(
                    idx as i32 * column_width as i32 + (column_width / 2) as i32,
                    window_height as i32 / 2,
                ),
                column_width,
                height,
            ))?;
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.render_frame(canvas)?;
        self.draw_minimap(canvas, (WINDOW_WIDTH as f32 / 5., WINDOW_WIDTH as f32 / 5.))?;
        Ok(())
    }
}

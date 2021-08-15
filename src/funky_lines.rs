use palette::encoding::Linear;
use palette::gradient::named;
use palette::rgb::Rgb;
use palette::*;
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{self, MouseButton};
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Canvas, TextureCreator};
use sdl2::video::Window;
use std::cmp::max;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const COL_FACTOR: usize = 1024;
const NUM_POINTS: usize = 50;

fn get_color(iteration: usize) -> Color {
    let gradient = named::VIRIDIS;
    let mut ratio = ((iteration % (COL_FACTOR * 2)) as f32 / COL_FACTOR as f32);
    if (ratio > 1.) {
        ratio = 1. - (ratio - ratio.floor())
    }
    let col = gradient.get(ratio);
    let [r, g, b]: [f32; 3] = *col.as_raw();
    Color::RGB((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}

struct Points {
    pub velocity: (f64, f64),
    pub points: VecDeque<Point>,
    pub last_point: (f64, f64),
    pub bounds: (u32, u32),
}
impl Points {
    fn random(bounds: (u32, u32)) -> Self {
        let mut rng = thread_rng();
        let mut randoms = [0f64; 4];
        rng.fill(&mut randoms[..]);

        let mut backwards = [false; 2];
        rng.fill(&mut backwards[..]);

        let vx = randoms[2] * if (backwards[0]) { -1. } else { 1. };
        let vy = randoms[3] * if (backwards[1]) { -1. } else { 1. };
        let velocity = (vx, vy);

        let px = bounds.0 as f64 * randoms[0];
        let py = bounds.1 as f64 * randoms[1];

        let points = vec![Point::new((px).round() as i32, (py).round() as i32)].into();
        Self {
            velocity,
            points,
            last_point: (px, py),
            bounds,
        }
    }

    fn update_velocity(&mut self, point: &Point) {
        let (x, y) = (point.x(), point.y());
        let (w, h) = self.bounds;
        if (x <= 0 || x >= w as i32) {
            self.velocity.0 = -self.velocity.0
        } else if (y <= 0 || y >= h as i32) {
            self.velocity.1 = -self.velocity.1
        }
    }

    fn generate_next_point(&mut self) -> Point {
        let mut x = self.last_point.0 + self.velocity.0;
        let mut y = self.last_point.1 + self.velocity.1;

        let mut rng = thread_rng();

        // clamp position
        if x < 0. {
            x = rng.gen_range(0..self.bounds.0) as f64;
        } else if x > self.bounds.0 as f64 {
            x = rng.gen_range(0..self.bounds.0) as f64;
        }
        if y < 0. {
            y = rng.gen_range(0..self.bounds.1) as f64;
        } else if y > self.bounds.1 as f64 {
            y = rng.gen_range(0..self.bounds.1) as f64;
        }

        self.last_point = (x, y);
        let point = Point::new((x).round() as i32, (y).round() as i32);

        self.points.push_back(point.clone());
        self.update_velocity(&point);
        return point;
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.draw_points(&*self.points.make_contiguous())
    }
}

fn generate_initial_points(bounds: (u32, u32), num_points: usize) -> Vec<Points> {
    println!("Generating {} points.", num_points);
    let points = (0..num_points)
        .map(|i| {
            if (i % 100 == 0) {
                print!("\r{} / {}...", i, num_points);
            }
            Points::random(bounds)
        })
        .collect();
    println!("\r{0} / {0}: Done!", num_points);
    points
}

fn update_scale_factor(scale_factor: f32, y: i32) -> f32 {
    match (y, scale_factor - 1.0 < std::f32::EPSILON) {
        (-1, true) => scale_factor / 2.,
        (-1, false) => scale_factor - 1.,
        (1, true) => scale_factor * 2.,
        (1, false) => scale_factor + 1.,
        _ => scale_factor,
    }
}

fn update_bounds(points: &mut Vec<Points>, new_bounds: (u32, u32)) {
    points
        .iter_mut()
        .for_each(|mut points| points.bounds = new_bounds);
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    sdl_context.mouse().show_cursor(false);
    let window = video_subsystem
        .window("bouncy lines", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut scale_factor = 1.0;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_scale(scale_factor, scale_factor)?;
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let (w, h) = canvas.window().size();
    let mut iteration = 0;

    let get_bounds = |canvas: &Canvas<Window>| {
        let (w, h) = canvas.window().size();
        let (sw, sh) = canvas.scale();
        (
            ((w as f32) * (1. / sw)) as u32,
            ((h as f32) * (1. / sh)) as u32,
        )
    };

    let mut bounds = get_bounds(&canvas);

    let mut pause = false;

    let mut points_master = generate_initial_points(bounds, NUM_POINTS);

    'running: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Left => points_master.push(Points::random(bounds)),
                    MouseButton::Right => {
                        points_master.pop();
                    }
                    _ => {}
                },

                Event::MouseWheel { y, .. } => {
                    scale_factor = update_scale_factor(scale_factor, y);
                    canvas.set_scale(scale_factor, scale_factor)?;

                    bounds = get_bounds(&canvas);
                    update_bounds(&mut points_master, bounds);

                    canvas.set_draw_color(Color::BLACK);
                    canvas.clear();
                    println!("set_scale_factor({})", scale_factor);
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Space => pause = !pause,
                    Keycode::C => {
                        canvas.set_draw_color(Color::BLACK);
                        canvas.clear()
                    }
                    _ => {}
                },

                _ => {}
            }
        }
        println!("Event time {}ms", start.elapsed().as_millis());

        if (!pause) {
            let start = Instant::now();
            for mut points in points_master.iter_mut() {
                canvas.set_draw_color(get_color(iteration));
                let new_point = points.generate_next_point();
                canvas.draw_point(new_point)?;
            }
            println!("Frame time {}ms", start.elapsed().as_millis());
            iteration += 1;
        }
        let start = Instant::now();
        canvas.present();
        println!("Present time {}ms", start.elapsed().as_millis());
    }

    Ok(())
}

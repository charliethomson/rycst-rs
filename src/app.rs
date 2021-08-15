use std::time::Instant;

use factor::factor::factor;
use lazy_static::lazy_static;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    ttf::{self, FontStyle},
    video::Window,
    EventPump,
};

use crate::{state::State, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(PartialEq, Eq)]
enum ControlFlow {
    Continue,
    Break,
}

lazy_static! {
    pub static ref INTERVAL: usize = {
        let width_factors = factor(WINDOW_WIDTH as i64);
        let height_factors = factor(WINDOW_HEIGHT as i64);

        let common_factors: Vec<i64> = width_factors
            .into_iter()
            .filter(|current| height_factors.contains(current))
            .collect();
        common_factors[3 * common_factors.len() / 4] as usize
    };
}

pub fn get_canvas_size(canvas: &Canvas<Window>) -> (u32, u32) {
    let (w, h) = canvas.window().size();
    let (sw, sh) = canvas.scale();
    (
        ((w as f32) * (1. / sw)) as u32,
        ((h as f32) * (1. / sh)) as u32,
    )
}

#[allow(unused)]
struct AppConfig {
    max_fps: Option<u8>,
    show_fps_counter: bool,
}
#[allow(dead_code)]
impl AppConfig {
    pub fn set_max_fps(mut self, max_fps: u8) -> Self {
        if max_fps == 0 {
            panic!("Max FPS cannot be zero, stop.")
        };
        self.max_fps = Some(max_fps);
        self
    }
    pub fn no_fps_limit(mut self) -> Self {
        self.max_fps = None;
        self
    }
    pub fn show_fps_counter(mut self, show_fps_counter: bool) -> Self {
        self.show_fps_counter = show_fps_counter;
        self
    }
    fn wait_for_frame(&self, start: &Instant) {
        if let Some(fps) = self.max_fps {
            let time = (1. / fps as f32 * 1000000.).round() as u128;
            while start.elapsed().as_micros() < time {}
        }
    }
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_fps: Some(60),
            show_fps_counter: true,
        }
    }
}

pub struct App {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    state: State,
    cfg: AppConfig,
}
impl App {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        sdl_context.mouse().set_relative_mouse_mode(true);

        let window = video_subsystem
            .window("raycasting", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        let state = State::new();

        Ok(Self {
            canvas,
            event_pump,
            state,
            cfg: AppConfig::default().set_max_fps(144),
        })
    }

    fn handle_events(&mut self) -> ControlFlow {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return ControlFlow::Break,
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.state.key_up(keycode);
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.state.key_down(keycode);
                }

                Event::MouseMotion { xrel, .. } => self.state.mouse_motion(xrel),

                _ => {}
            }
        }

        ControlFlow::Continue
    }

    fn draw_fps(&mut self, fps: f64) -> Result<(), String> {
        let font_context = ttf::init().unwrap();
        let mut font = font_context.load_font("./assets/font/Pixeboy.ttf", 20)?;
        font.set_style(FontStyle::NORMAL);

        let fps_str = &format!("{:.0}", fps);
        let fs = font.size_of(fps_str).unwrap();
        self.canvas.set_draw_color(Color::BLACK);

        self.canvas.fill_rect(Rect::new(
            (WINDOW_WIDTH - fs.0 * 4) as i32,
            0,
            fs.0 * 6,
            fs.1 * 5,
        ))?;

        let texture_creator = self.canvas.texture_creator();

        self.canvas.copy(
            &font
                .render(fps_str)
                .solid(Color::WHITE)
                .unwrap()
                .as_texture(&texture_creator)
                .unwrap(),
            None,
            Some(Rect::new(
                (WINDOW_WIDTH - (fs.0 as f32 * 3.25) as u32) as i32,
                fs.1 as i32,
                fs.0 * 3,
                fs.1 * 3,
            )),
        )?;
        Ok(())
    }

    pub fn start(mut self) -> Result<(), String> {
        'running: loop {
            let start = Instant::now();

            if self.handle_events() == ControlFlow::Break {
                break 'running;
            };

            self.state.update();

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.state.draw(&mut self.canvas)?;

            self.cfg.wait_for_frame(&start);
            if self.cfg.show_fps_counter {
                let fps = 1.0 / start.elapsed().as_secs_f64();
                self.draw_fps(fps)?;
            }
            self.canvas.present();
        }
        Ok(())
    }
}

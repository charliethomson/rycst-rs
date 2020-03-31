
use coffee::{
    graphics::{
        Point,
    },
    input::{
        Event,
        keyboard::KeyCode,
        mouse::Button as MouseButton,
        ButtonState,
        Input,
        mouse::Event as MouseEvent,
        keyboard::Event as KBEvent,
    },
};

use std::collections::HashMap;

use crate::config::{ WIDTH, HEIGHT };

#[derive(Clone)]
pub struct InputHandler {
    pub mouse_pos: Point,
    pub keys: HashMap<KeyCode, ButtonState>,
    pub mouse: HashMap<MouseButton, ButtonState>,
    pub buf: String,
    pub win_dims: (f32, f32),
    pub mouse_moved: Option<(f32, f32)>, // (dx, dy)
} impl InputHandler {

    pub fn is_pressed(&self, key: &KeyCode) -> bool {
        Some(&ButtonState::Pressed) == self.keys.get(key)
    }

} impl Input for InputHandler {
    fn new() -> Self {
        Self {
            mouse_pos: Point::origin(),
            keys: HashMap::new(),
            mouse: HashMap::new(),
            buf: String::new(),
            mouse_moved: None,
            win_dims: (WIDTH as f32, HEIGHT as f32),
        }
    }

    fn clear(&mut self) {
        self.mouse_moved = None;
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::Keyboard(kb_ev) => {
                match kb_ev {
                    KBEvent::Input { state, key_code } => {
                        self.keys.insert(key_code, state);
                    },
                    KBEvent::TextEntered { character } => {
                        self.buf.push(character);
                    }
                }
            },
            Event::Mouse(m_ev) => {
                match m_ev {
                    MouseEvent::CursorMoved { x, y } => {
                        self.mouse_moved = Some((self.mouse_pos.x - x, self.mouse_pos.y - y));
                        self.mouse_pos = Point::new(x, y);
                    },
                    MouseEvent::Input { state, button } => {
                        self.mouse.insert(button, state);
                    },
                    _ => (),
                }
            },
            Event::Window(_win_ev) => {
                // TODO: Focused / Unfocused
            },
            _ => (),
        }
    }
}


use coffee::{
    Game,
    Timer,
    load::{
        Task,
    },
    graphics::{
        Window,
        Frame,
        Shape,
        Point,
        Color,
        Rectangle,
    },
    input::{ 
        keyboard::KeyCode,
        ButtonState,
    },
    ui::Text,
};

use crate::{
    input::InputHandler,
    render::{ Renderer, ShapeExt },
    util::Direction,
};


pub struct Engine {

    renderer: Renderer,

} impl Game for Engine {

    type Input = InputHandler;
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Self> {
        Task::new(|| 
            // TODO: Add options, possibly a load / save menu, map making tool
        
            Self {
                renderer: Renderer::new(),
            }
        )
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let scene = self.renderer.render(&Rectangle {
            x: 100.0,
            y: 100.0,
            width: 400.0,
            height: 400.0,
        });

        scene.draw(&mut frame.as_target());


        let map = self.renderer.render_top_down(&Rectangle {
            x: 10.0,
            y: 10.0, 
            width: 190.0,
            height: 190.0,
        });

        map.draw(&mut frame.as_target());

    }

    fn interact(&mut self, input: &mut InputHandler, _window: &mut Window) {
        if input.is_pressed(&KeyCode::W) {
            self.renderer.mv(Direction::Forward);
        }
        
        if input.is_pressed(&KeyCode::A) {
            self.renderer.mv(Direction::Left);
        }
        
        if input.is_pressed(&KeyCode::S) {
            self.renderer.mv(Direction::Backward);
        }
        
        if input.is_pressed(&KeyCode::D) {
            self.renderer.mv(Direction::Right);
        }

        if input.is_pressed(&KeyCode::Right) {
            self.renderer.rotate(Direction::Right);
        }
        
        if input.is_pressed(&KeyCode::Left) {
            self.renderer.rotate(Direction::Left);
        }
        
    }
}
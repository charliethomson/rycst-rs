

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
    input::{ Input, KeyboardAndMouse },
    ui::Text,
};
use crate::{
    input::InputHandler,
    player::Player,
    render::{ Renderer, ShapeExt },
};


pub struct Engine {

    player: Player,
    renderer: Renderer,
    input: InputHandler,

} impl Game for Engine {

    type Input = InputHandler;
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Self> {
        Task::new(|| 
            // TODO: Add options, possibly a load / save menu, map making tool
        
            Self {
                player: Player {},
                renderer: Renderer::new(),
                input: InputHandler::new(),
            }
        )
    }

    fn draw(&mut self, _frame: &mut Frame, _timer: &Timer) {
        _frame.clear(Color::BLACK);
        // let rects = vec![
        //     Shape::rect(Point::new(100.0, 100.0), 100.0, 100.0),
        //     Shape::rect(Point::new(600.0, 100.0), 100.0, 100.0),
        //     Shape::rect(Point::new(600.0, 600.0), 100.0, 100.0),
        //     Shape::rect(Point::new(100.0, 600.0), 100.0, 100.0),
        // ];

        // let mut mesh = coffee::graphics::Mesh::new();

        // for rect in rects.iter().cloned() {
        //     if let Shape::Rectangle(r) = rect {
        //         let col = if r.contains(self.input.mouse_pos) {
        //             if self.input.mouse.get(&coffee::input::mouse::Button::Left) == Some(&coffee::input::ButtonState::Pressed) {
        //                 Color::from_rgb(0, 255, 0)
        //             } else {
        //                 Color::from_rgb(0, 0, 255)
        //             }
        //         } else {
        //             Color::from_rgb(255, 0, 0)
        //         };
        //         mesh.fill(rect, col)
        //     }
        // }

        // mesh.draw(&mut _frame.as_target());

        // if let Some((dx, _)) = self.input.mouse_moved {
        //     eprintln!("{}", dx);
        //     self.renderer.rotate(dx);
        // }
        // eprintln!("renderer angle: {:?}", self.renderer.angle());



        let scene = self.renderer.render(&Rectangle {
            x: 0.0, 
            y: 0.0,
            width: 800.0,
            height: 800.0,
        });

        scene.draw(&mut _frame.as_target());

        let map = self.renderer.render_top_down(&Rectangle {
            x: 10.0,
            y: 10.0, 
            width: 200.0,
            height: 200.0,
        });
        map.draw(&mut _frame.as_target());

    }

    fn interact(&mut self, input: &mut InputHandler, _window: &mut Window) {
        self.input = input.clone();
    }
}
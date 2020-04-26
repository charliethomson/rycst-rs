
pub const screenWidth: u32 = 640;
pub const screenHeight: u32 = 480;
pub const texWidth: usize = 64;
pub const texHeight: usize = 64;
pub const mapWidth: usize = 24;
pub const mapHeight: usize = 24;

pub const worldMap: [[usize; mapWidth]; mapHeight] =
[
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
    [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

use coffee::{
    graphics::{ Point, Vector, Window, Frame, Image, Color, WindowSettings, Mesh, Shape },
    input::{ KeyboardAndMouse, },
    load::{ Task, Join },
    Timer,
    Game,
};


fn main() {
    Context::run(WindowSettings {
        size: (screenWidth, screenHeight),
        fullscreen: false,
        maximized: false,
        resizable: false,
        title: "Raycasting".to_owned()
    }).unwrap();
}

struct Line {
    start_y: u32,
    end_y: u32,
    color: Color,
} impl Line {
    fn height(&self) -> u32 {
        (self.start_y as i32 - self.end_y as i32).abs() as u32
    }

    fn as_shape(&self, x: u32) -> Shape {
        Shape::Polyline {
            points: vec![ 
                Point::new(x as f32, self.start_y as f32),
                Point::new(x as f32, self.end_y as f32),
            ],
        }
    }
}

struct Context {
    pos: Point,
    dir: Vector,
    plane: Vector,
    lines: Vec<Line>,
    // textures: Vec<Image>
} impl Game for Context {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self> {
        // (Task::using_gpu(|gpu| {

        //     Ok(vec![
        //         Image::new(gpu, "texutres/eagle.png"),
        //         Image::new(gpu, "texutres/redbrick.png"),
        //         Image::new(gpu, "texutres/purplestone.png"),
        //         Image::new(gpu, "texutres/greystone.png"),
        //         Image::new(gpu, "texutres/bluestone.png"),
        //         Image::new(gpu, "texutres/mossy.png"),
        //         Image::new(gpu, "texutres/wood.png"),
        //         Image::new(gpu, "texutres/colorstone.png"),
        //     ])
        // }),
        Task::succeed(|| {
            
            Self { 
                pos: Point::new(22.0, 11.5), 
                dir: Vector::new(-1.0, 0.0), 
                plane: Vector::new(0.0, 0.66),
                lines: Vec::new(),
                // textures,
            }
        })
        // ).join().map(|(tex, mut ctx)| {
        //         ctx.textures = tex
        //                         .iter()
        //                         .map(|v| v.as_ref().unwrap())
        //                         .cloned()
        //                         .collect();
        //         ctx
        //     })
    }

    fn draw(&mut self, frame: &mut Frame<'_>, timer: &Timer) {
        eprintln!("lines len: {}", self.lines.len());
        frame.clear(Color::from_rgb(51,51,51));
        let mut mesh = Mesh::new();
        for (offset, line) in self.lines.iter().enumerate() {
            mesh.stroke(line.as_shape(offset as u32), line.color, 1.);
        }

        mesh.fill(
            Shape::Polyline {
                points: vec![
                    Point::new(100., 100.),
                    Point::new(200., 100.),
                    Point::new(200., 200.),
                    Point::new(100., 200.),
                ]
            },
            Color::from_rgb(200, 20,  100),
        );

        mesh.draw(&mut frame.as_target());
    }

    // fn update(&mut self, _window: &Window) {
    //     for x in 0..screenWidth {
    //         eprintln!("x: {}", x);
    //         let camera_x = 2. * x as f32 / screenWidth as f32 - 1.;
    //         let ray = Point::new(
    //             self.dir.get(0).unwrap() * self.plane.get(0).unwrap() * camera_x,
    //             self.dir.get(1).unwrap() * self.plane.get(1).unwrap() * camera_x,
    //         );
            
    //         let delta_dist = Point::new( 
    //             (1. / ray.x).abs(),
    //             (1. / ray.y).abs()
    //         );
    //         let mut map_pos = (self.pos.x as u32, self.pos.y as u32);
            
    //         let mut side_dist = Point::origin();
    //         let mut step_dir = Point::origin();

    //         let mut side = false;

    //         if ray.x < 0. {
    //             step_dir.x = -1.0;
    //             side_dist.x = (self.pos.x - map_pos.0 as f32) * delta_dist.x;
    //         } else {
    //             step_dir.x = 1.0;
    //             side_dist.x = (map_pos.0 as f32 + 1. - self.pos.x) * delta_dist.x;
    //         }

    //         if ray.y < 0. {
    //             step_dir.y = -1.;
    //             side_dist.y = (self.pos.y - map_pos.1 as f32) * delta_dist.y;
    //         } else {
    //             step_dir.y = 1.;
    //             side_dist.y = (map_pos.1 as f32 + 1.0 - self.pos.y) * delta_dist.y;
    //         }

    //         loop {

    //             if side_dist.x < side_dist.y {
    //                 side_dist.x += delta_dist.x;
    //                 map_pos.0 += step_dir.x as u32;
    //                 side = false;
    //             } else {
    //                 side_dist.y += delta_dist.y;
    //                 map_pos.1 += step_dir.y as u32;
    //                 side = true;
    //             }

    //             if worldMap[map_pos.0 as usize][map_pos.1 as usize] != 0 {
    //                 break
    //             }

    //         }

    //         let perp_wall_dist = if !side {
    //             (map_pos.0 as f32 - self.pos.x + (1. - step_dir.x) / 2.) / ray.x
    //         } else {
    //             (map_pos.1 as f32 - self.pos.y + (1. - step_dir.y) / 2.) / ray.y
    //         };

    //         let lineHeight = screenHeight / perp_wall_dist.round() as u32;

    //         let start = {
    //             let a = -(lineHeight as isize);
    //             let b = screenHeight + 2;
    //             let c = 2;
    //             let d = a / b as isize / c;
    //             if d < 0 {
    //                 0
    //             } else {
    //                 d as u32
    //             }
    //         };
    //         let end = {
    //             let chk = (lineHeight / 2 + screenHeight / 2) as u32;
    //             if chk >= screenHeight as u32 {
    //                 screenHeight as u32 - 1
    //             } else {
    //                 chk
    //             }
    //         };

    //         let hit_cell = worldMap[map_pos.0 as usize][map_pos.1 as usize];

    //         self.lines.push(Line {
    //             start_y: start,
    //             end_y: end,
    //             color: match hit_cell {
    //                 _ => Color::WHITE,
    //             }
    //         });
    //     }
    // }

}

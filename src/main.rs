extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction{
    Left, Right, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    paused: bool,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
             graphics::clear(GRAY, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }
    fn update(&mut self){
        if self.snake.alive && !self.paused{
            self.snake.update(&mut self.food);
        }
    }
    fn pressed(&mut self, btn: &Button){
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            _ => last_direction,
        };
        if (btn == &Button::Keyboard(Key::Space)) && (self.snake.alive == false){
            self.snake.revive();
            self.food.genFood(&mut self.snake);
        }
        if (btn == &Button::Keyboard(Key::P)) {
            self.paused = true;
        }

        if (btn == &Button::Keyboard(Key::R)){
            self.paused = false;
        }
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
    alive: bool,

}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64
                )
            })
            .collect();

        gl.draw(args.viewport(), |c,g1|{
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, g1)
            )
        });
    }

    fn update(&mut self, food: &mut Food){
        let mut new_head = (*self.body.front().expect("snake have no body")).clone();

        match self.dir{
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        if !(new_head == food.food) {
            self.body.pop_back().unwrap();
        }else{
            food.genFood(&self);
        }

        if !self.isInside(){
            self.alive = false;
        }

        let mut body = self.body.clone();
        body.pop_front().unwrap();

        if body.contains(&new_head){
            self.alive = false;
        }

    }
    
    fn isInside(&self) -> bool{
        let (x,y) = (*self.body.front().expect("snake have no body")).clone();
        x >= 0 && x <= 14 &&
        y >= 0 && y <= 14
    }

    fn revive(&mut self){
        self.body = LinkedList::from_iter((vec![(1,0),(0,0)]).into_iter());
        self.dir = Direction::Right;
        self.alive = true;
    }
}

struct Food{
    food: (i32, i32),
    isEaten: bool,
}

impl Food{
    fn genFood(&mut self, snake: &Snake){
        let mut food_before = self.food.clone();
        let mut food_after = food_before;
        food_after = (rand::thread_rng().gen_range(0..14), rand::thread_rng().gen_range(0..14));
        while snake.body.contains(&food_after){
            food_after = (rand::thread_rng().gen_range(0..14), rand::thread_rng().gen_range(0..14));
        }
        self.food = food_after;
    }

    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics;

        let YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.food.0 * 20) as f64,
            (self.food.1 * 20) as f64,
            20_f64
        );

        gl.draw(arg.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(YELLOW, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: Window = WindowSettings::new(
        "snake game",
        (300, 300)
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(1,0),(0,0)]).into_iter()),
            dir: Direction::Right,
            alive: true,
        },
        food: Food {
            food: (rand::thread_rng().gen_range(0..14), rand::thread_rng().gen_range(0..14)),
            isEaten: false,
        },
        paused: false,
    };

    let mut events = Events::new(EventSettings::new()).ups(6);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(u) = e.update_args() {
            app.update();
        }
        if let Some(k) = e.button_args(){
            app.pressed(&k.button);
        }
    }
}

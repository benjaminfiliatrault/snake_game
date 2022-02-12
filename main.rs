extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use graphics::color::{NAVY, RED, TEAL};
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston::{
    event_loop::*, Button, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateEvent,
};
use rand::Rng;
use std::collections::LinkedList;
use std::iter::FromIterator;

// Window configurations
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 600;
const BACKGROUND_COLOR: Color = TEAL;

// Snake configuration
const SNAKE_COLOR: Color = NAVY;
const SNAKE_SIZE: f64 = 20_f64;

const FOOD_COLOR: Color = RED;
const FOOD_SIZE: f64 = 10_f64;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);
        });
        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update(self.food.clone());
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction.ne(&Direction::Down) => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction.ne(&Direction::Up) => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction.ne(&Direction::Right) => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction.ne(&Direction::Left) => {
                Direction::Right
            }
            _ => last_direction,
        };
    }
}

#[derive(Clone, PartialEq)]
struct Food {
    pos_x: i32,
    pos_y: i32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let pos_x = self.pos_x;
        let pos_y = self.pos_y;

        let square =
            graphics::rectangle::square((pos_x * 20) as f64, (pos_y * 20) as f64, FOOD_SIZE);

        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(FOOD_COLOR, square, c.transform, gl);
        })
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, SNAKE_SIZE)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(SNAKE_COLOR, square, transform, gl))
        })
    }

    fn update(&mut self, food: Food) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        let head_x = new_head.0;
        let head_y = new_head.1;

        if head_x == food.pos_x && head_y == food.pos_y {
            self.body.push_back(new_head);
        }

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,

            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [WINDOW_HEIGHT, WINDOW_WIDTH])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
        food: Food {
            pos_x: rand::thread_rng().gen_range(1, WINDOW_HEIGHT / 20) as i32,
            pos_y: rand::thread_rng().gen_range(1, WINDOW_WIDTH / 20) as i32,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);

    // Game loop right down here
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_args) = e.update_args() {
            game.update()
        }

        if let Some(btn_args) = e.button_args() {
            if btn_args.state == ButtonState::Press {
                game.pressed(&btn_args.button);
            }
        }
    }
}

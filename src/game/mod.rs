use graphics::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, ButtonState, MouseButton, RenderArgs};

use crate::game::draw::DrawData;
use crate::UpdateArgs;

mod cell;
pub(crate) mod draw;
pub(crate) mod field;
pub(crate) mod menu;

pub const CELL_SIZE: u32 = 30;

pub struct FieldSize {
    pub width: u32,
    pub height: u32,
    pub mines: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Game {
    field: field::Field,
    menu: menu::Menu,
    mouse_position: Option<Point<f64>>,
    game_started: bool,
    game_over: bool,
    win: bool,
}

impl Game {
    pub fn new(size: FieldSize) -> Game {
        Game {
            field: field::Field::new(size),
            menu: menu::Menu::new(),
            mouse_position: None,
            game_started: false,
            game_over: false,
            win: false,
        }
    }
}

impl Game {
    pub fn height(&self) -> u32 {
        self.field.height() + self.menu.height()
    }

    pub fn width(&self) -> u32 {
        self.field.width()
    }

    pub fn game_started(&self) -> bool {
        self.game_started
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn win(&self) -> bool {
        self.win
    }
}

impl GameElement for Game {
    fn render(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData) {
        self.menu.render(&args, c, gl, dd);
        let field_render_context = Context {
            transform: c.transform.trans(0.0, self.menu.height() as f64),
            ..c
        };
        self.field.render(&args, field_render_context, gl, dd);
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.field.update(&args);
        self.menu.update(&args);
    }
}

impl Game {
    pub fn mouse_move(&mut self, mouse_args: &[f64]) {
        self.mouse_position = Some(Point {
            x: mouse_args[0],
            y: mouse_args[1],
        });
    }

    pub fn button_press(&mut self, args: &ButtonArgs) {
        if let Some(point) = self.mouse_position {
            if point.y < self.menu.height() as f64 {
                self.menu.button_press(args, point);
            } else {
                let cell_point = Point {
                    x: (point.x / CELL_SIZE as f64) as u32,
                    y: ((point.y - self.menu.height() as f64) / CELL_SIZE as f64) as u32,
                };

                if args.state == ButtonState::Release
                    && args.button == Button::from(MouseButton::Left)
                    && !self.game_started
                {
                    self.game_started = true;
                    self.field.init(cell_point);
                }
                self.field.button_press(args, cell_point);
            }
        }
    }
}

pub trait GameElement {
    fn render(&self, render_args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData);
    fn update(&mut self, update_args: &UpdateArgs);
}

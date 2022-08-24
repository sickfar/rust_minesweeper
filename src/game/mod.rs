use graphics::color::WHITE;
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

pub const CELL_SIZE: f64 = 30.0;

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

impl Point<u32> {
    pub fn to_f64(&self) -> Point<f64> {
        Point {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    Ready,
    Playing,
    Win,
    Loose,
}

pub enum CellPressResult {
    NoAction,
    Opened,
    Flagged,
    Unflagged,
    Exploded,
}

pub struct Game {
    field: field::Field,
    menu: menu::Menu,
    mouse_position: Option<Point<f64>>,
    game_state: GameState,
}

impl Game {
    pub fn new(size: FieldSize) -> Game {
        let field = field::Field::new(size);
        let width = field.width() as f64;
        let mines = field.mines();
        Game {
            field,
            menu: menu::Menu::new(width, mines),
            mouse_position: None,
            game_state: GameState::Ready,
        }
    }
}

impl Game {
    pub fn height(&self) -> f64 {
        self.field.height() + self.menu.height()
    }

    pub fn width(&self) -> f64 {
        self.field.width()
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }
}

impl GameElement for Game {
    fn render(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData) {
        graphics::clear(WHITE, gl);
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
                let result = self.menu.button_press(args, point);
                match result {
                    menu::MenuButtonPressResult::NewGame => {
                        self.field.reset();
                        self.menu.set_mines(self.field.mines());
                        self.game_state = GameState::Ready;
                    }
                    menu::MenuButtonPressResult::NoAction => {}
                }
            } else {
                if self.game_state == GameState::Ready || self.game_state == GameState::Playing {
                    let result = self.button_press_field(args, point);
                    match result {
                        CellPressResult::Exploded => {
                            self.game_state = GameState::Loose;
                            self.menu.stop_timer();
                        }
                        CellPressResult::Flagged | CellPressResult::Unflagged => {
                            self.menu.set_mines(self.field.mines() - self.field.flags());
                        }
                        CellPressResult::Opened => {}
                        _ => {}
                    }
                }
            }
        }
    }

    fn button_press_field(&mut self, args: &ButtonArgs, point: Point<f64>) -> CellPressResult {
        let cell_point = Point {
            x: (point.x / CELL_SIZE as f64) as u32,
            y: ((point.y - self.menu.height() as f64) / CELL_SIZE as f64) as u32,
        };

        if args.state == ButtonState::Release
            && args.button == Button::from(MouseButton::Left)
            && self.game_state == GameState::Ready
        {
            self.game_state = GameState::Playing;
            self.field.init(cell_point);
            self.menu.start_timer();
            self.menu.set_mines(self.field.mines());
        }
        self.field.button_press(args, cell_point)
    }
}

pub trait GameElement {
    fn render(&self, render_args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData);
    fn update(&mut self, update_args: &UpdateArgs);
}

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

pub enum CellInteractionResult {
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
    button_press_counter: u8,
    both_buttons_flag: bool,
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
            button_press_counter: 0,
            both_buttons_flag: false,
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

    pub fn button_action(&mut self, args: &ButtonArgs) {
        if let Some(point) = self.mouse_position {
            if point.y < self.menu.height() as f64 {
                let result = self.menu.button_action(args, point);
                match result {
                    menu::MenuButtonPressResult::NewGame => {
                        self.switch_state(GameState::Ready);
                    }
                    menu::MenuButtonPressResult::NoAction => {}
                }
            } else {
                if self.game_state == GameState::Ready || self.game_state == GameState::Playing {
                    let result = self.button_action_field(args, point);
                    match result {
                        CellInteractionResult::Exploded => {
                            self.switch_state(GameState::Loose);
                            self.both_buttons_flag = false;
                        }
                        CellInteractionResult::Flagged | CellInteractionResult::Unflagged => {
                            self.menu.set_mines(self.field.mines() - self.field.flags());
                        }
                        CellInteractionResult::Opened => {
                            self.both_buttons_flag = false;
                            if self.field.cells_left() == 0 {
                                self.switch_state(GameState::Win);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn switch_state(&mut self, state: GameState) {
        self.end_current_state();
        self.enter_new_state(state)
    }

    fn end_current_state(&mut self) {
        match self.game_state {
            GameState::Ready => {}
            GameState::Playing => {
                self.menu.stop_timer();
                self.button_press_counter = 0;
                self.both_buttons_flag = false;
            }
            GameState::Win => {}
            GameState::Loose => {}
        }
    }

    fn enter_new_state(&mut self, state: GameState) {
        self.game_state = state;
        match self.game_state {
            GameState::Ready => {
                self.field.reset();
                self.menu.set_mines(self.field.mines());
                self.menu.set_ok();
            }
            GameState::Playing => {
                self.menu.start_timer();
                self.menu.set_mines(self.field.mines());
                self.menu.set_ok();
            }
            GameState::Win => {
                self.menu.set_win();
            }
            GameState::Loose => {
                self.menu.set_loose();
            }
        }
    }

    fn button_action_field(
        &mut self,
        args: &ButtonArgs,
        point: Point<f64>,
    ) -> CellInteractionResult {
        let cell_point = Point {
            x: (point.x / CELL_SIZE as f64) as u32,
            y: ((point.y - self.menu.height() as f64) / CELL_SIZE as f64) as u32,
        };

        if args.state == ButtonState::Release
            && args.button == Button::from(MouseButton::Left)
            && self.game_state == GameState::Ready
        {
            self.field.init(cell_point);
            self.enter_new_state(GameState::Playing);
        }
        if args.state == ButtonState::Press
            && (args.button == Button::from(MouseButton::Left)
                || args.button == Button::from(MouseButton::Right))
        {
            self.button_press_counter += 1;
            if self.button_press_counter == 2 {
                self.both_buttons_flag = true;
            }
        } else if args.state == ButtonState::Release
            && (args.button == Button::from(MouseButton::Left)
                || args.button == Button::from(MouseButton::Right))
        {
            self.button_press_counter -= 1;
        }
        self.field.button_action(
            args,
            cell_point,
            self.both_buttons_flag && self.button_press_counter == 1,
        ) //this means unpressed last button
    }
}

pub trait GameElement {
    fn render(&self, render_args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData);
    fn update(&mut self, update_args: &UpdateArgs);
}

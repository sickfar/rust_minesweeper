use crate::game::draw::{
    draw_counter, draw_menu_button, draw_menu_button_pressed, draw_timer, DrawData, Rect,
};
use crate::game::Point;
use crate::GameElement;
use graphics::color::BLACK;
use graphics::types::Rectangle;
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, ButtonState, MouseButton, RenderArgs, UpdateArgs};

pub const MENU_HEIGHT: f64 = 50.0;
const MENU_BLUE: [f32; 4] = [0.273, 0.384, 0.940, 1.0];

pub enum MenuButtonPressResult {
    NoAction,
    NewGame,
}

struct GameButton {
    rect: Rect,
    pressed: bool,
}

struct Timer {
    time: f64,
    runnnig: bool,
    rect: Rect,
}

struct MineCounter {
    mines: u32,
    rect: Rect,
}

pub struct Menu {
    rect: Rect,
    game_button: GameButton,
    timer: Timer,
    mine_counter: MineCounter,
}

impl Menu {
    pub fn new(width: f64, mines: u32) -> Menu {
        Menu {
            rect: Rect::new(0.0, 0.0, width, MENU_HEIGHT),
            game_button: GameButton {
                rect: Rect::square(
                    width / 2.0 - MENU_HEIGHT * 0.4,
                    MENU_HEIGHT * 0.1,
                    MENU_HEIGHT * 0.8,
                ),
                pressed: false,
            },
            timer: Timer {
                time: 0.0,
                runnnig: false,
                rect: Rect::new(
                    MENU_HEIGHT * 0.1,
                    MENU_HEIGHT * 0.1,
                    MENU_HEIGHT * 2.0,
                    MENU_HEIGHT * 0.8,
                ),
            },
            mine_counter: MineCounter {
                mines,
                rect: Rect::new(
                    width - MENU_HEIGHT * 0.1 - MENU_HEIGHT * 2.0,
                    MENU_HEIGHT * 0.1,
                    MENU_HEIGHT * 2.0,
                    MENU_HEIGHT * 0.8,
                ),
            },
        }
    }
}

impl Menu {
    pub fn height(&self) -> f64 {
        MENU_HEIGHT
    }
}

//events
impl Menu {
    pub fn button_press(
        &mut self,
        button_args: &ButtonArgs,
        point: Point<f64>,
    ) -> MenuButtonPressResult {
        if button_args.state == ButtonState::Release {
            if button_args.button == Button::from(MouseButton::Left) {
                if self.game_button.rect.contains_point(point) {
                    self.game_button.pressed = false;
                    self.timer.runnnig = false;
                    self.timer.time = 0.0;
                    return MenuButtonPressResult::NewGame;
                }
            }
        } else if button_args.state == ButtonState::Press {
            if button_args.button == Button::from(MouseButton::Left) {
                if self.game_button.rect.contains_point(point) {
                    self.game_button.pressed = true;
                }
            }
        }
        MenuButtonPressResult::NoAction
    }
}

impl Menu {
    pub fn start_timer(&mut self) {
        self.timer.runnnig = true;
    }

    pub fn stop_timer(&mut self) {
        self.timer.runnnig = false;
    }

    pub fn set_mines(&mut self, mines: u32) {
        self.mine_counter.mines = mines;
    }
}

impl GameElement for Menu {
    fn render(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData) {
        graphics::rectangle(MENU_BLUE, self.rect.to_arr(), c.transform, gl);
        graphics::line_from_to(
            BLACK,
            2.0,
            [0.0, self.rect.height - 1.0],
            [self.rect.width, self.rect.height - 1.0],
            c.transform,
            gl,
        );
        self.game_button.render(args, c, gl, dd);
        self.timer.render(args, c, gl, dd);
        self.mine_counter.render(args, c, gl, dd);
    }

    fn update(&mut self, _update_args: &UpdateArgs) {
        self.timer.update(_update_args);
    }
}

impl GameElement for GameButton {
    fn render(
        &self,
        _render_args: &RenderArgs,
        c: Context,
        gl: &mut GlGraphics,
        _dd: &mut DrawData,
    ) {
        if self.pressed {
            draw_menu_button_pressed(self.rect, c, gl);
        } else {
            draw_menu_button(self.rect, c, gl);
        }
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

impl GameElement for Timer {
    fn render(
        &self,
        _render_args: &RenderArgs,
        c: Context,
        gl: &mut GlGraphics,
        dd: &mut DrawData,
    ) {
        draw_timer(self.rect, self.time, c, gl, dd);
    }

    fn update(&mut self, update_args: &UpdateArgs) {
        if self.runnnig && self.time < 999.0 {
            self.time += update_args.dt;
        }
    }
}

impl GameElement for MineCounter {
    fn render(
        &self,
        _render_args: &RenderArgs,
        c: Context,
        gl: &mut GlGraphics,
        dd: &mut DrawData,
    ) {
        draw_counter(self.rect, self.mines, c, gl, dd);
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

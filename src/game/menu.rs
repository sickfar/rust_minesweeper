use crate::game::draw::{draw_menu_button, DrawData, Rect};
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
}

struct Timer {
    time: u32,
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
    pub fn new(width: f64) -> Menu {
        Menu {
            rect: Rect::new(0.0, 0.0, width, MENU_HEIGHT),
            game_button: GameButton {
                rect: Rect::square(
                    width / 2.0 - MENU_HEIGHT * 0.4,
                    MENU_HEIGHT * 0.1,
                    MENU_HEIGHT * 0.8,
                ),
            },
            timer: Timer {
                time: 0,
                rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            },
            mine_counter: MineCounter {
                mines: 0,
                rect: Rect::new(0.0, 0.0, 0.0, 0.0),
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
                    return MenuButtonPressResult::NewGame;
                }
            }
        }
        MenuButtonPressResult::NoAction
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
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

impl GameElement for GameButton {
    fn render(
        &self,
        _render_args: &RenderArgs,
        c: Context,
        gl: &mut GlGraphics,
        _dd: &mut DrawData,
    ) {
        draw_menu_button(self.rect, c, gl);
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

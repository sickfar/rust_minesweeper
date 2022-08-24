use crate::game::draw::DrawData;
use crate::game::Point;
use crate::GameElement;
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

pub const MENU_HEIGHT: u32 = 30;

struct GameButton {}

pub struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        Menu {}
    }
}

impl Menu {
    pub fn height(&self) -> u32 {
        MENU_HEIGHT
    }
}

//events
impl Menu {
    pub fn mouse_move(&mut self, mouse_args: &[f64]) {}

    pub fn button_press(&mut self, button_args: &ButtonArgs, point: Point<f64>) {}
}

impl GameElement for Menu {
    fn render(
        &self,
        _render_args: &RenderArgs,
        _c: Context,
        _gl: &mut GlGraphics,
        _dd: &mut DrawData,
    ) {
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

use crate::UpdateArgs;
use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs};

mod cell;
pub(crate) mod field;

pub const CELL_SIZE: u32 = 20;

pub const MINE_COUNT: u32 = 10;

pub struct FieldSize {
    pub width: u32,
    pub height: u32,
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub trait GameElement {
    fn render(&self, render_args: &RenderArgs, gl: &mut GlGraphics);
    fn update(&mut self, update_args: &UpdateArgs);
    fn button_press(&mut self, button_args: &ButtonArgs);
}

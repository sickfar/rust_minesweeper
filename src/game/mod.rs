use piston::RenderArgs;

use crate::game::draw::DrawData;
use crate::UpdateArgs;

mod cell;
pub(crate) mod draw;
pub(crate) mod field;

pub const CELL_SIZE: u32 = 30;

pub struct FieldSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub trait GameElement {
    fn render(&self, render_args: &RenderArgs, dd: &mut DrawData);
    fn update(&mut self, update_args: &UpdateArgs);
}

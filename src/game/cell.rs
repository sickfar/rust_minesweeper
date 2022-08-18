use crate::game::{GameElement, Point};
use crate::UpdateArgs;
use graphics::Graphics;
use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs};

pub enum CellState {
    Closed,
    Pressed,
    Opened,
    Flagged,
}

pub enum CellContent {
    Empty,
    Mine,
    Number(u8),
}

pub struct Cell {
    pub state: CellState,
    pub content: CellContent,
    pub position: Point,
}

impl Cell {
    pub fn new(point: Point) -> Cell {
        Cell {
            state: CellState::Closed,
            content: CellContent::Empty,
            position: point,
        }
    }

    pub fn set_mine(&mut self) {
        self.content = CellContent::Mine;
    }

    pub fn set_number(&mut self, number: u8) {
        self.content = CellContent::Number(number);
    }
}

impl GameElement for Cell {
    fn render(&self, _render_args: &RenderArgs, _gl: &mut GlGraphics) {}
    fn update(&mut self, _update_args: &UpdateArgs) {}

    fn button_press(&mut self, _button_args: &ButtonArgs) {}
}

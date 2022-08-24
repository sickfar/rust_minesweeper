use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::game::draw::DrawData;
use crate::game::{GameElement, Point};
use crate::UpdateArgs;

#[derive(PartialEq, Copy, Clone)]
pub enum CellState {
    Closed,
    Opened,
    Flagged,
}

#[derive(PartialEq)]
pub enum CellContent {
    Empty,
    Mine,
    Number(u8),
}

pub struct Cell {
    state: CellState,
    content: CellContent,
    position: Point<u32>,
}

//reads
impl Cell {
    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn position(&self) -> Point<u32> {
        self.position
    }

    pub fn is_mine(&self) -> bool {
        self.content == CellContent::Mine
    }

    pub fn is_empty(&self) -> bool {
        self.content == CellContent::Empty
    }
}

impl Cell {
    pub fn new(point: Point<u32>) -> Cell {
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
        if number > 0 {
            self.content = CellContent::Number(number);
        } else {
            self.content = CellContent::Empty;
        }
    }

    pub fn open(&mut self) {
        self.state = CellState::Opened;
    }

    pub fn flag(&mut self) {
        self.state = CellState::Flagged;
    }

    pub fn unflag(&mut self) {
        self.state = CellState::Closed;
    }
}

impl GameElement for Cell {
    fn render(&self, _: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData) {
        let glyph_cache = &mut dd.glyph_cache;
        match self.state {
            CellState::Opened => super::draw::draw_opened_cell(
                self.position.to_f64(),
                &self.content,
                &c,
                gl,
                glyph_cache,
            ),
            CellState::Flagged => super::draw::draw_flagged_cell(self.position.to_f64(), &c, gl),
            _ => super::draw::draw_closed_cell(self.position.to_f64(), &c, gl),
        }
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

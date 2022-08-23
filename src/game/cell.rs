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
    position: Point,
}

//reads
impl Cell {
    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn position(&self) -> Point {
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
    fn render(&self, render_args: &RenderArgs, dd: &mut DrawData) {
        let gl = &mut dd.gl;
        let glyph_cache = &mut dd.glyph_cache;
        gl.draw(render_args.viewport(), |c, gl| match self.state {
            CellState::Opened => {
                super::draw::draw_opened_cell(&self.position, &self.content, &c, gl, glyph_cache)
            }
            CellState::Flagged => super::draw::draw_flagged_cell(&self.position, &c, gl),
            _ => super::draw::draw_closed_cell(&self.position, &c, gl),
        })
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

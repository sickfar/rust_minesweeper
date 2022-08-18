use crate::game::cell::Cell;
use crate::game::{FieldSize, GameElement, Point};
use opengl_graphics::GlGraphics;
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

pub const FIELD_SIZE_10: FieldSize = FieldSize {
    width: 8,
    height: 8,
};
pub const FIELD_SIZE_40: FieldSize = FieldSize {
    width: 13,
    height: 15,
};
pub const FIELD_SIZE_99: FieldSize = FieldSize {
    width: 16,
    height: 30,
};

pub struct Field {
    pub cells: Vec<Vec<Cell>>,
    pub size: FieldSize,
    pub mines: u32,
}

impl Field {
    pub fn new(size: FieldSize, mines: u32) -> Field {
        let mut cells = Vec::new();
        for col_i in 0..size.height {
            let mut row = Vec::new();
            for row_i in 0..size.width {
                row.push(Cell::new(Point { x: col_i, y: row_i }));
            }
            cells.push(row);
        }
        Field { cells, size, mines }
    }
}

impl GameElement for Field {
    fn render(&self, render_args: &RenderArgs, gl: &mut GlGraphics) {
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = &self.cells[row as usize][col as usize];
                cell.render(render_args, gl);
            }
        }
    }
    fn update(&mut self, update_args: &UpdateArgs) {
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = &mut self.cells[row as usize][col as usize];
                cell.update(update_args);
            }
        }
    }

    fn button_press(&mut self, button_args: &ButtonArgs) {
        todo!()
    }
}

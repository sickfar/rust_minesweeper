use crate::game::cell::Cell;
use crate::game::{FieldSize, GameElement, Point};
use crate::CELL_SIZE;
use graphics::color::WHITE;
use opengl_graphics::GlGraphics;
use piston::Input::Button;
use piston::{ButtonArgs, MouseButton, RenderArgs, UpdateArgs};

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
    mouse_position: Point,
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
        Field {
            cells,
            size,
            mines,
            mouse_position: Point { x: 0, y: 0 },
        }
    }

    pub fn button_press(&mut self, button_args: &ButtonArgs) {
        let cell = &mut self.cells[self.mouse_position.x as usize][self.mouse_position.y as usize];
        cell.button_press(button_args);
    }

    pub fn mouse_move(&mut self, mouse_args: &[f64]) {
        self.mouse_position = Point {
            x: (mouse_args[0] / CELL_SIZE as f64) as u32,
            y: (mouse_args[1] / CELL_SIZE as f64) as u32,
        };
    }
}

impl GameElement for Field {
    fn render(&self, render_args: &RenderArgs, gl: &mut GlGraphics) {
        graphics::clear(WHITE, gl);
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
}

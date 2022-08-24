use graphics::color::WHITE;
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, ButtonState, MouseButton, RenderArgs, UpdateArgs};
use rand::Rng;

use crate::game::cell::{Cell, CellState};
use crate::game::draw::DrawData;
use crate::game::{FieldSize, GameElement, Point};
use crate::CELL_SIZE;

pub const FIELD_SIZE_10: FieldSize = FieldSize {
    width: 8,
    height: 8,
    mines: 10,
};
pub const FIELD_SIZE_40: FieldSize = FieldSize {
    width: 13,
    height: 15,
    mines: 40,
};
pub const FIELD_SIZE_99: FieldSize = FieldSize {
    width: 16,
    height: 30,
    mines: 99,
};

pub struct Field {
    rows: Vec<Vec<Cell>>,
    size: FieldSize,
    mines: u32,
    flags: u32,
}

// public getters
impl Field {
    pub fn height(&self) -> u32 {
        self.size.height * CELL_SIZE
    }

    pub fn width(&self) -> u32 {
        self.size.width * CELL_SIZE
    }

    pub fn mines(&self) -> u32 {
        self.mines
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}

// private getters
impl Field {
    fn cell_at_point(&self, point: Point<u32>) -> &Cell {
        self.cell_at(point.x, point.y)
    }

    fn mut_cell_at_point(&mut self, point: Point<u32>) -> &mut Cell {
        self.mut_cell_at(point.x, point.y)
    }

    fn cell_at(&self, x: u32, y: u32) -> &Cell {
        &self.rows[y as usize][x as usize]
    }

    fn mut_cell_at(&mut self, x: u32, y: u32) -> &mut Cell {
        &mut self.rows[y as usize][x as usize]
    }

    fn get_neighbours(&self, point: Point<u32>) -> Vec<Point<u32>> {
        let mut neighbours = Vec::new();
        if point.x > 0 {
            neighbours.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.x < self.size.width - 1 {
            neighbours.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            neighbours.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.y < self.size.height - 1 {
            neighbours.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }
        if point.x > 0 && point.y > 0 {
            neighbours.push(Point {
                x: point.x - 1,
                y: point.y - 1,
            });
        }
        if point.x < self.size.width - 1 && point.y > 0 {
            neighbours.push(Point {
                x: point.x + 1,
                y: point.y - 1,
            });
        }
        if point.x > 0 && point.y < self.size.height - 1 {
            neighbours.push(Point {
                x: point.x - 1,
                y: point.y + 1,
            });
        }
        if point.x < self.size.width - 1 && point.y < self.size.height - 1 {
            neighbours.push(Point {
                x: point.x + 1,
                y: point.y + 1,
            });
        }
        neighbours
    }
}

// init
fn generate_rows(size: &FieldSize) -> Vec<Vec<Cell>> {
    let mut rows = Vec::new();
    for row_i in 0..size.height {
        let mut col = Vec::new();
        for col_i in 0..size.width {
            col.push(Cell::new(Point { x: col_i, y: row_i }));
        }
        rows.push(col);
    }
    rows
}

impl Field {
    pub fn new(size: FieldSize) -> Field {
        let mut rows = generate_rows(&size);
        let field = Field {
            rows,
            mines: size.mines,
            size,
            flags: 0,
        };
        field
    }

    pub fn init(&mut self, except_pos: Point<u32>) {
        self.rows = generate_rows(&self.size);
        self.randomize_mines(except_pos);
        let numbers = self.calculate_numbers();
        self.assign_numbers(numbers);
    }

    fn randomize_mines(&mut self, except_pos: Point<u32>) {
        let mut rng = rand::thread_rng();
        let mut mines = self.mines;
        while mines > 0 {
            let y = rng.gen_range(0..self.size.height);
            let x = rng.gen_range(0..self.size.width);
            let cell = self.mut_cell_at(x, y);
            if y != except_pos.y && x != except_pos.x && !cell.is_mine() {
                cell.set_mine();
                mines -= 1;
            }
        }
    }

    fn calculate_numbers(&self) -> Vec<Vec<u8>> {
        let mut numbers = vec![vec![0; self.size.width as usize]; self.size.height as usize];
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                let cell = self.cell_at(x, y);
                if !cell.is_mine() {
                    let neighbours = self.get_neighbours(cell.position());
                    let mut mine_count = 0;
                    for neighbour in neighbours {
                        if self.cell_at(neighbour.x, neighbour.y).is_mine() {
                            mine_count += 1;
                        }
                    }
                    numbers[y as usize][x as usize] = mine_count;
                }
            }
        }
        numbers
    }

    fn assign_numbers(&mut self, numbers: Vec<Vec<u8>>) {
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                if !self.cell_at(x, y).is_mine() {
                    self.mut_cell_at(x, y)
                        .set_number(numbers[y as usize][x as usize]);
                }
            }
        }
    }

    fn open_neighbours(&mut self, point: Point<u32>) {
        let neighbours = self.get_neighbours(point);
        for neighbour in neighbours {
            if self.cell_at(neighbour.x, neighbour.y).state() == CellState::Closed {
                self.mut_cell_at(neighbour.x, neighbour.y).open();
                if self.cell_at(neighbour.x, neighbour.y).is_empty() {
                    self.open_neighbours(neighbour);
                }
            }
        }
    }
}

// events
impl Field {
    pub fn button_press(&mut self, button_args: &ButtonArgs, cell_point: Point<u32>) {
        if button_args.state == ButtonState::Release {
            if button_args.button == Button::from(MouseButton::Left) {
                if self.cell_at_point(cell_point).state() == CellState::Closed {
                    self.mut_cell_at_point(cell_point).open();
                    if self.cell_at_point(cell_point).is_empty() {
                        self.open_neighbours(self.cell_at_point(cell_point).position());
                    }
                }
            } else if button_args.button == Button::from(MouseButton::Right) {
                if self.cell_at_point(cell_point).state() == CellState::Closed {
                    self.mut_cell_at_point(cell_point).flag();
                    self.flags += 1;
                } else if self.cell_at_point(cell_point).state() == CellState::Flagged {
                    self.mut_cell_at_point(cell_point).unflag();
                }
            }
        }
    }
}

impl GameElement for Field {
    fn render(&self, render_args: &RenderArgs, c: Context, gl: &mut GlGraphics, dd: &mut DrawData) {
        graphics::clear(WHITE, gl);
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = &self.rows[row as usize][col as usize];
                cell.render(render_args, c, gl, dd);
            }
        }
    }
    fn update(&mut self, update_args: &UpdateArgs) {
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = &mut self.rows[row as usize][col as usize];
                cell.update(update_args);
            }
        }
    }
}

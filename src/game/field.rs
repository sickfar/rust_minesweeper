use crate::game::cell::{Cell, CellContent};
use crate::game::draw::DrawData;
use crate::game::{FieldSize, GameElement, Point};
use crate::CELL_SIZE;
use graphics::color::WHITE;
use opengl_graphics::GlGraphics;
use piston::Input::Button;
use piston::{ButtonArgs, MouseButton, RenderArgs, UpdateArgs};
use rand::Rng;

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
    pub rows: Vec<Vec<Cell>>,
    pub size: FieldSize,
    pub mines: u32,
    mouse_position: Point,
}

impl Field {
    pub fn new(size: FieldSize, mines: u32) -> Field {
        let mut rows = Vec::new();
        for row_i in 0..size.height {
            let mut col = Vec::new();
            for col_i in 0..size.width {
                col.push(Cell::new(Point { x: col_i, y: row_i }));
            }
            rows.push(col);
        }
        let mut field = Field {
            rows,
            size,
            mines,
            mouse_position: Point { x: 0, y: 0 },
        };
        field.randomize_mines();
        let numbers = field.calculate_numbers();
        field.assign_numbers(numbers);
        field
    }

    fn randomize_mines(&mut self) {
        let mut rng = rand::thread_rng();
        let mut mines = self.mines;
        while mines > 0 {
            let row = rng.gen_range(0..self.size.height) as usize;
            let col = rng.gen_range(0..self.size.width) as usize;
            if self.rows[row][col].content != CellContent::Mine {
                self.rows[row][col].set_mine();
                mines -= 1;
            }
        }
    }

    fn get_neighbours(&self, point: &Point) -> Vec<Point> {
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

    fn calculate_numbers(&self) -> Vec<Vec<u8>> {
        let mut numbers = vec![vec![0; self.size.width as usize]; self.size.height as usize];
        for row_i in 0..self.size.height as usize {
            for col_i in 0..self.size.width as usize {
                let cell = &self.rows[row_i][col_i];
                if cell.content != CellContent::Mine {
                    let neighbours = self.get_neighbours(&cell.position);
                    let mut mine_count = 0;
                    for neighbour in neighbours {
                        if self.rows[neighbour.y as usize][neighbour.x as usize].content
                            == CellContent::Mine
                        {
                            mine_count += 1;
                        }
                    }
                    numbers[row_i][col_i] = mine_count;
                }
            }
        }
        numbers
    }

    fn assign_numbers(&mut self, numbers: Vec<Vec<u8>>) {
        for row_i in 0..self.size.height as usize {
            for col_i in 0..self.size.width as usize {
                if self.rows[row_i][col_i].content != CellContent::Mine {
                    self.rows[row_i][col_i].set_number(numbers[row_i as usize][col_i as usize]);
                }
            }
        }
    }

    pub fn button_press(&mut self, button_args: &ButtonArgs) {
        let cell = &mut self.rows[self.mouse_position.y as usize][self.mouse_position.x as usize];
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
    fn render(&self, render_args: &RenderArgs, dd: &mut DrawData) {
        graphics::clear(WHITE, &mut dd.gl);
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = &self.rows[row as usize][col as usize];
                cell.render(render_args, dd);
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

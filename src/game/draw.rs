use graphics::math::Matrix2d;
use graphics::{Context, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};

use crate::game::cell::CellContent;
use crate::game::Point;
use crate::CELL_SIZE;

pub struct DrawData<'a> {
    pub glyph_cache: GlyphCache<'a>,
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn square(x: f64, y: f64, size: f64) -> Rect {
        Rect {
            x,
            y,
            width: size,
            height: size,
        }
    }

    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_arr(arr: [f64; 4]) -> Rect {
        Rect::new(arr[0], arr[1], arr[2], arr[3])
    }

    pub fn from_arr_u(arr: [u32; 4]) -> Rect {
        Rect::new(arr[0] as f64, arr[1] as f64, arr[2] as f64, arr[3] as f64)
    }

    pub fn from_cell_point_f64(point: Point<f64>) -> Rect {
        Rect::new(
            point.x * CELL_SIZE,
            point.y * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        )
    }

    pub fn from_cell_point_u32(point: Point<u32>) -> Rect {
        Rect::new(
            point.x as f64 * CELL_SIZE,
            point.y as f64 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        )
    }

    pub fn to_arr(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }

    pub fn coord_arr(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn contains_point(&self, point: Point<f64>) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

const CELL_BLUE: [f32; 4] = [0.0, 0.6352941176470588, 0.9098039215686275, 1.0];
const LIGHT_BLUE: [f32; 4] = [0.6, 0.8509803921568627, 0.9176470588235294, 1.0];
const DARK_BLUE: [f32; 4] = [
    0.2588235294117647,
    0.392156862745098,
    0.5764705882352941,
    1.0,
];
const GRAY: [f32; 4] = [
    0.7647058823529412,
    0.7647058823529412,
    0.7647058823529412,
    1.0,
];
const DARK_RED: [f32; 4] = [0.5333333333333333, 0.0, 0.0823529411764706, 1.0];

const TEXT_COLORS: [[f32; 4]; 8] = [
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0, 1.0],
];

fn draw_bordered_square_0(
    color: [f32; 4],
    top_color: [f32; 4],
    right_color: [f32; 4],
    left_color: [f32; 4],
    bottom_color: [f32; 4],
    rect: Rect,
    transform: Matrix2d,
    gl: &mut GlGraphics,
) {
    graphics::rectangle(color, rect.to_arr(), transform, gl);
    graphics::line_from_to(
        top_color,
        1.0,
        [rect.x + 1.0, rect.y],
        [rect.x + rect.width - 1.0, rect.y],
        transform,
        gl,
    );
    graphics::line_from_to(
        right_color,
        1.0,
        [rect.x + rect.width - 1.0, rect.y],
        [rect.x + rect.width - 1.0, rect.y + rect.height - 1.0],
        transform,
        gl,
    );
    graphics::line_from_to(
        left_color,
        1.0,
        [rect.x, rect.y + 1.0],
        [rect.x, rect.y + rect.height],
        transform,
        gl,
    );
    graphics::line_from_to(
        bottom_color,
        1.0,
        [rect.x, rect.y + rect.height - 1.0],
        [rect.x + rect.width - 1.0, rect.y + rect.height - 1.0],
        transform,
        gl,
    );
}

fn draw_closed_cell_0(position: Point<f64>, transform: Matrix2d, gl: &mut GlGraphics) {
    draw_bordered_square_0(
        CELL_BLUE,
        LIGHT_BLUE,
        LIGHT_BLUE,
        DARK_BLUE,
        DARK_BLUE,
        Rect {
            x: position.x * CELL_SIZE,
            y: position.y * CELL_SIZE,
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        transform,
        gl,
    );
}

fn draw_flag_0(position: Point<f64>, gl: &mut GlGraphics, transform: Matrix2d) {
    graphics::line_from_to(
        GRAY,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0) as f64,
            (position.y * CELL_SIZE + 5.0) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 5.0) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        DARK_RED,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0 + 1.0) as f64,
            (position.y * CELL_SIZE + 5.0) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0 + 10.0) as f64,
            (position.y * CELL_SIZE + 11.0) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        DARK_RED,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0 + 10.0) as f64,
            (position.y * CELL_SIZE + 11.0) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2.0 - 3.0 + 1.0) as f64,
            (position.y * CELL_SIZE + 18.0) as f64,
        ],
        transform,
        gl,
    );
}

pub fn draw_closed_cell(position: Point<f64>, c: &Context, gl: &mut GlGraphics) {
    draw_closed_cell_0(position, c.transform, gl)
}

pub fn draw_flagged_cell(position: Point<f64>, c: &Context, gl: &mut GlGraphics) {
    draw_closed_cell_0(position, c.transform, gl);
    draw_flag_0(position, gl, c.transform);
}

pub fn draw_opened_cell(
    position: Point<f64>,
    content: &CellContent,
    c: &Context,
    gl: &mut GlGraphics,
    glyph_cache: &mut GlyphCache,
) {
    let transform = c.transform;
    draw_bordered_square_0(
        GRAY,
        DARK_BLUE,
        DARK_BLUE,
        DARK_BLUE,
        DARK_BLUE,
        Rect::from_cell_point_f64(position),
        transform,
        gl,
    );
    match content {
        CellContent::Mine => {
            draw_mine_0(position, gl, transform);
        }
        CellContent::Number(number) => {
            draw_number_0(position, gl, glyph_cache, transform, number);
        }
        _ => {} //nothing
    }
}

fn draw_number_0(
    position: Point<f64>,
    gl: &mut GlGraphics,
    glyph_cache: &mut GlyphCache,
    transform: Matrix2d,
    number: &u8,
) {
    graphics::text(
        TEXT_COLORS[(number - 1) as usize],
        (CELL_SIZE as f64 * 0.7) as u32,
        number.to_string().as_str(),
        glyph_cache,
        transform.trans(
            position.x * CELL_SIZE + (CELL_SIZE / 4.0),
            position.y * CELL_SIZE + (CELL_SIZE / 5.0 * 4.0),
        ),
        gl,
    )
    .expect("Cell text should be rendered");
}

fn draw_mine_0(position: Point<f64>, gl: &mut GlGraphics, transform: Matrix2d) {
    graphics::circle_arc(
        DARK_RED,
        1.0,
        0.0,
        360.0,
        [2.0, 2.0, (CELL_SIZE - 4.0) as f64, (CELL_SIZE - 4.0) as f64],
        transform.trans(position.x * CELL_SIZE, position.y * CELL_SIZE),
        gl,
    );
}

pub fn draw_menu_button(rect: Rect, c: Context, gl: &mut GlGraphics) {
    draw_bordered_square_0(
        CELL_BLUE,
        LIGHT_BLUE,
        LIGHT_BLUE,
        DARK_BLUE,
        DARK_BLUE,
        rect,
        c.transform,
        gl,
    );
}

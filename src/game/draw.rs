use crate::game::cell::CellContent;
use crate::game::Point;
use crate::CELL_SIZE;
use graphics::math::Matrix2d;
use graphics::types::Color;
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.6352941176470588, 0.9098039215686275, 1.0];
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

fn draw_closed_cell_0(position: &Point, transform: Matrix2d, gl: &mut GlGraphics) {
    let square = graphics::rectangle::square(
        (position.x * CELL_SIZE) as f64,
        (position.y * CELL_SIZE) as f64,
        CELL_SIZE as f64,
    );
    graphics::rectangle(BLUE, square, transform, gl);
    graphics::line_from_to(
        LIGHT_BLUE,
        1.0,
        [
            (position.x * CELL_SIZE) as f64,
            (position.y * CELL_SIZE) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
            (position.y * CELL_SIZE) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        LIGHT_BLUE,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
            (position.y * CELL_SIZE) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 2) as f64,
        ],
        transform,
        gl,
    );

    graphics::line_from_to(
        DARK_BLUE,
        1.0,
        [
            (position.x * CELL_SIZE) as f64,
            (position.y * CELL_SIZE + 1) as f64,
        ],
        [
            (position.x * CELL_SIZE) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        DARK_BLUE,
        1.0,
        [
            (position.x * CELL_SIZE) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
        ],
        transform,
        gl,
    );
}

fn draw_flag_0(position: &Point, gl: &mut GlGraphics, transform: Matrix2d) {
    graphics::line_from_to(
        GRAY,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3) as f64,
            (position.y * CELL_SIZE + 5) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3) as f64,
            (position.y * CELL_SIZE + CELL_SIZE - 5) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        DARK_RED,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3 + 1) as f64,
            (position.y * CELL_SIZE + 5) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3 + 10) as f64,
            (position.y * CELL_SIZE + 11) as f64,
        ],
        transform,
        gl,
    );
    graphics::line_from_to(
        DARK_RED,
        1.0,
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3 + 10) as f64,
            (position.y * CELL_SIZE + 11) as f64,
        ],
        [
            (position.x * CELL_SIZE + CELL_SIZE / 2 - 3 + 1) as f64,
            (position.y * CELL_SIZE + 18) as f64,
        ],
        transform,
        gl,
    );
}

pub fn draw_closed_cell(position: &Point, render_args: &RenderArgs, gl: &mut GlGraphics) {
    gl.draw(render_args.viewport(), |c, gl| {
        draw_closed_cell_0(position, c.transform, gl)
    });
}

pub fn draw_flagged_cell(position: &Point, render_args: &RenderArgs, gl: &mut GlGraphics) {
    gl.draw(render_args.viewport(), |c, gl| {
        let transform = c.transform;
        draw_closed_cell_0(position, transform, gl);
        draw_flag_0(position, gl, transform);
    });
}

pub fn draw_opened_cell(
    position: &Point,
    content: &CellContent,
    render_args: &RenderArgs,
    gl: &mut GlGraphics,
) {
    gl.draw(render_args.viewport(), |c, gl| {
        let transform = c.transform;
        let square = graphics::rectangle::square(
            (position.x * CELL_SIZE) as f64,
            (position.y * CELL_SIZE) as f64,
            CELL_SIZE as f64,
        );
        graphics::rectangle(GRAY, square, transform, gl);
        graphics::line_from_to(
            DARK_BLUE,
            1.0,
            [
                (position.x * CELL_SIZE) as f64,
                (position.y * CELL_SIZE) as f64,
            ],
            [
                (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
                (position.y * CELL_SIZE) as f64,
            ],
            transform,
            gl,
        );
        graphics::line_from_to(
            DARK_BLUE,
            1.0,
            [
                (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
                (position.y * CELL_SIZE) as f64,
            ],
            [
                (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
                (position.y * CELL_SIZE + CELL_SIZE - 2) as f64,
            ],
            transform,
            gl,
        );

        graphics::line_from_to(
            DARK_BLUE,
            1.0,
            [
                (position.x * CELL_SIZE) as f64,
                (position.y * CELL_SIZE + 1) as f64,
            ],
            [
                (position.x * CELL_SIZE) as f64,
                (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
            ],
            transform,
            gl,
        );
        graphics::line_from_to(
            DARK_BLUE,
            1.0,
            [
                (position.x * CELL_SIZE) as f64,
                (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
            ],
            [
                (position.x * CELL_SIZE + CELL_SIZE - 1) as f64,
                (position.y * CELL_SIZE + CELL_SIZE - 1) as f64,
            ],
            transform,
            gl,
        );
        match content {
            CellContent::Empty => {}
            CellContent::Mine => {}
            CellContent::Number(number) => {}
        }
    });
}

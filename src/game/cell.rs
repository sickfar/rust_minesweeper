use crate::game::draw::DrawData;
use crate::game::{GameElement, Point};
use crate::{UpdateArgs, CELL_SIZE};
use base64::decode;
use graphics::color::BLUE;
use graphics::{Context, Graphics, Image};
use image::io::Reader;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::{Button, ButtonArgs, ButtonState, MouseButton, RenderArgs};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

#[derive(PartialEq)]
pub enum CellState {
    Closed,
    Pressed,
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
        if number > 0 {
            self.content = CellContent::Number(number);
        } else {
            self.content = CellContent::Empty;
        }
    }

    pub fn button_press(&mut self, button_args: &ButtonArgs) {
        if button_args.state == ButtonState::Release {
            if button_args.button == Button::from(MouseButton::Left) {
                if self.state == CellState::Closed {
                    self.state = CellState::Opened;
                }
            } else if button_args.button == Button::from(MouseButton::Right) {
                if self.state == CellState::Closed {
                    self.state = CellState::Flagged;
                } else if self.state == CellState::Flagged {
                    self.state = CellState::Closed;
                }
            }
        }
    }
}

impl GameElement for Cell {
    fn render(&self, render_args: &RenderArgs, dd: &mut DrawData) {
        let gl = &mut dd.gl;
        let glyph_cache = &mut dd.glyph_cache;
        gl.draw(render_args.viewport(), |c, gl| match self.state {
            CellState::Opened => super::draw::draw_opened_cell(
                &self.position,
                &self.content,
                render_args,
                gl,
                glyph_cache,
            ),
            CellState::Flagged => super::draw::draw_flagged_cell(&self.position, render_args, gl),
            _ => super::draw::draw_closed_cell(&self.position, render_args, gl),
        })
    }

    fn update(&mut self, _update_args: &UpdateArgs) {}
}

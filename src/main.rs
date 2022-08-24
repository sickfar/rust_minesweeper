extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod game;

use crate::game::{GameElement, CELL_SIZE};
use glutin_window::GlutinWindow as Window;
use graphics::glyph_cache::rusttype::GlyphCache;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, MouseCursorEvent};

fn main() {
    let opengl = OpenGL::V3_2;

    let field_size = game::field::FIELD_SIZE_40;
    let mut game = game::Game::new(field_size);

    let mut window: Window = WindowSettings::new("Rust Minesweeper", [game.width(), game.height()])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut draw_data = game::draw::DrawData {
        glyph_cache: GlyphCache::new("assets/Roboto-Regular.ttf", (), TextureSettings::new())
            .unwrap(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                game.render(&args, c, gl, &mut draw_data);
            });
        }
        if let Some(args) = e.update_args() {
            game.update(&args);
        }
        if let Some(args) = e.mouse_cursor_args() {
            game.mouse_move(&args);
        }
        if let Some(args) = e.button_args() {
            game.button_press(&args);
        }
    }
}

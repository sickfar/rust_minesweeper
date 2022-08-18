extern crate base64;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;
mod resources;

use crate::game::{GameElement, CELL_SIZE};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, Event};

fn main() {
    let opengl = OpenGL::V3_2;

    let field_size = game::field::FIELD_SIZE_10;

    // Create a Glutin window.
    let mut window: GlutinWindow = WindowSettings::new(
        "Rust Minesweeper",
        [field_size.width * CELL_SIZE, field_size.height * CELL_SIZE],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut field = game::field::Field::new(field_size, 10);
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            field.render(&args, &mut gl);
        }

        if let Some(args) = e.update_args() {
            field.update(&args);
        }

        if let Some(args) = e.button_args() {
            field.button_press(&args);
        }
    }
}
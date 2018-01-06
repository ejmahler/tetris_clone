#![feature(universal_impl_trait)]
#![feature(nll)]

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;

extern crate piston_window;
extern crate piston;
extern crate image;
extern crate cgmath;

extern crate num_traits;

extern crate rand;

use piston_window::WindowSettings;

mod engine;
mod tetris;

fn main() {
    let mut window = WindowSettings::new("Dark Tetris", [640, 480]).exit_on_esc(true).build().unwrap();
    let mut app = tetris::TetrisApp::new(&mut window);

    engine::exec(&mut app);
}
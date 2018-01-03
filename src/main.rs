#![feature(universal_impl_trait)]

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;

extern crate piston_window;
extern crate piston;
extern crate cgmath;
extern crate num_traits;

use piston_window::WindowSettings;

mod core;
mod tetris;

fn main() {
    let mut window = WindowSettings::new("Dark Tetris", [640, 480]).exit_on_esc(true).build().unwrap();
    let mut app = tetris::TetrisApp::new(&mut window);

    core::exec(&mut app);
}
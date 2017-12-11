#![feature(universal_impl_trait)]

extern crate piston_window;
extern crate piston;
extern crate num_traits;

use piston_window::WindowSettings;

mod core;
mod tetris;

fn main() {
    let window = WindowSettings::new("Dark Tetris", [640, 480]).exit_on_esc(true).build().unwrap();
    let mut app = tetris::TetrisApp::new(window);

    core::exec(&mut app);
}
extern crate piston_window;
extern crate piston;
extern crate num_traits;

use piston::event_loop::Events;
use piston_window::{WindowSettings, Loop, Event};

mod app;
mod game_input;

use app::TetrisApp;

fn main() {
    let window = WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
    let mut app = TetrisApp::new(window);

    let mut events = Events::new(app.create_event_settings());

    while let Some(event) = events.next(app.get_window()) {
        match event {
            Event::Loop(Loop::Render(args)) => app.render(args),
            Event::Loop(Loop::AfterRender(args)) => app.after_render(args),
            Event::Loop(Loop::Update(args)) => app.update(args),
            Event::Loop(Loop::Idle(args)) => app.idle(args),
            Event::Input(args) => app.handle_input(args),
            other => println!("got unknown event: {:?}", other),
        }
    }
}